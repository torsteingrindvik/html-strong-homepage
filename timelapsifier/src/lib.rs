use std::{
    collections::BTreeMap,
    num::ParseIntError,
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};

use chrono::{DateTime, Local, TimeZone, Timelike};
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;
use tokio::{fs, io::AsyncWriteExt, process::Command, sync::RwLock};
use tracing::{debug, error, info, instrument, trace, warn};

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
  (?P<y>\d{4}) # the year
  -
  (?P<m>\d{2}) # the month
  -
  (?P<d>\d{2}) # the day
  _
  (?P<H>\d{2}) # the hour
  -
  (?P<M>\d{2}) # the minutes
  -
  (?P<S>\d{2}) # the seconds
  \.jpg
",
    )
    .expect("regex should compile")
});

#[derive(Debug, Clone)]
pub struct TimestampedFile {
    pub file: PathBuf,
    pub timestamp: DateTime<Local>,
}

impl AsRef<Path> for TimestampedFile {
    fn as_ref(&self) -> &Path {
        &self.file
    }
}

fn duration_until_tomorrow_night_01() -> std::time::Duration {
    let now = Local::now();

    // Add one day, then floor towards 01:00 in the night.
    let tomorrow_night_01 = (now + chrono::Duration::days(1))
        .with_hour(1)
        .expect("hour ok")
        .with_minute(0)
        .expect("minute ok")
        .with_second(0)
        .expect("seconds ok");

    tomorrow_night_01
        .signed_duration_since(now)
        .to_std()
        .expect("Duration std ok")
}

pub type StateVideos = Arc<RwLock<Vec<TimestampedFile>>>;

#[derive(Debug)]
pub struct TimelapserOptions {
    pub unprocessed_images_folder: PathBuf,
    pub processed_images_folder: PathBuf,
    pub timelapse_output_folder: PathBuf,

    pub timelapse_videos: StateVideos,
}

#[instrument]
async fn do_work(options: &TimelapserOptions) {
    info!("Looking for timelapse image candidates");

    let mut candidates = candidates(&options.unprocessed_images_folder).await;
    if candidates.is_empty() {
        info!("No candidates");
        return;
    }
    sort_files_by_timestamp(&mut candidates);

    info!("Found {} candidates", candidates.len());

    let before_today = images_before_day_of(candidates.clone(), Local::now());
    if before_today.is_empty() {
        info!("No images from earlier days");
        return;
    }

    let grouped_by_days = group_by_day(before_today);

    for (day, images_that_day) in grouped_by_days {
        info!("Making a timelapse for day {day:?}");

        let mut output_video = options.timelapse_output_folder.clone();
        if !output_video.exists() {
            info!("Making output dir {output_video:?}");
            fs::create_dir_all(&output_video)
                .await
                .expect("make dirs ok");
        }

        output_video.push(format!("{}.mp4", day.format("%Y-%m-%d")));

        let output = ffmpeg_make_clip(
            &images_that_day,
            output_video.to_str().expect("ok &str mp4 path"),
        )
        .await;

        if !output.status.success() {
            error!(?output, "Timelapse creation not successful!");
            return;
        } else {
            info!("Timelapse made ok: {output_video:?}")
        }

        if !options.processed_images_folder.exists() {
            info!(
                "Making processed dir {:?}",
                &options.processed_images_folder
            );
            fs::create_dir_all(&options.processed_images_folder)
                .await
                .expect("make dirs ok");
        }

        move_all(&images_that_day, &options.processed_images_folder).await;
        info!(
            "Images processed ({}) moved to processed folder.",
            images_that_day.len()
        );

        let videos = files_of_ext_in(&options.timelapse_output_folder, &["mp4"]).await;
        let num_videos = videos.len();
        info!("# mp4: {num_videos}");

        let timestamped_videos = videos
            .into_iter()
            .filter_map(|f| f.try_into().ok())
            .collect::<Vec<_>>();
        if timestamped_videos.len() != num_videos {
            warn!(
                "Could not figure out the timestamp of some mp4 files ({} out of {})",
                timestamped_videos.len(),
                num_videos
            );
        }

        *options.timelapse_videos.write().await = timestamped_videos;

        // TODO: Delete processed images >1 week old?
    }
}

/// Create a worker
pub fn spawn_worker(options: TimelapserOptions) {
    use tokio::time;

    tokio::spawn(async move {
        info!("Timelapsifying forever");

        loop {
            do_work(&options).await;

            let sleep_duration = duration_until_tomorrow_night_01();
            debug!("Sleeping for {:?}", sleep_duration);

            time::sleep(sleep_duration).await;
        }
    });
}

async fn ffmpeg_make_clip(images: &[TimestampedFile], output_mp4: &str) -> Output {
    let file_name = "images-input.txt";

    let mut f = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)
        .await
        .expect("should be able to create file");

    for image in images {
        let to_write = format!(
            "file '{}'\n",
            image
                .file
                .to_str()
                .expect("should be able to get file name")
        );

        f.write(to_write.as_bytes()).await.expect("write ok");
    }

    Command::new("ffmpeg")
        .args([
            "-y", "-f", "image2", "-r", "60", "-f", "concat", "-y", "-safe", "0", "-i", file_name,
            "-vcodec", "libx264", "-crf", "25", "-pix_fmt", "yuv420p", output_mp4,
        ])
        .output()
        .await
        .expect("Couldn't run ffmpeg properly")
}

/// Sort images such that earlier timestamped images
/// come first.
pub fn sort_files_by_timestamp(files: &mut [TimestampedFile]) {
    files.sort_unstable_by_key(|file| file.timestamp);
}

// In order to compare on a day-to-day basis, floor other components.
fn floor_to_day(timestamp: &DateTime<Local>) -> DateTime<Local> {
    timestamp
        .with_hour(0)
        .unwrap()
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
}

/// Takes a vector of sorted images, and returns all that came before the day of the given timestamp.
fn images_before_day_of(
    images: Vec<TimestampedFile>,
    timestamp: DateTime<Local>,
) -> Vec<TimestampedFile> {
    let timestamp = floor_to_day(&timestamp);

    images
        .into_iter()
        .take_while(|image| image.timestamp < timestamp)
        .collect()
}

/// Group a vector of sorted images into groups by the day.
fn group_by_day(images: Vec<TimestampedFile>) -> BTreeMap<DateTime<Local>, Vec<TimestampedFile>> {
    let mut groups = BTreeMap::<DateTime<Local>, Vec<TimestampedFile>>::new();

    for image in images {
        let t = floor_to_day(&image.timestamp);
        groups.entry(t).or_default().push(image);
    }

    groups
}

#[derive(Debug, Error)]
pub enum TimelapsifyError {
    #[error("Unexpected situation {0}")]
    Weird(&'static str),

    #[error("Unexpected situation {msg} in {rugrat:?}")]
    FileHuh { msg: &'static str, rugrat: PathBuf },

    #[error("No match")]
    RegexNoMatch,

    #[error("Parse oh no: {0:?}")]
    ParseOhNo(#[from] ParseIntError),
}

impl TimelapsifyError {
    fn file_option_issue(file: &Path) -> Self {
        Self::FileHuh {
            msg: "a method on a str-like which returns option failed",
            rugrat: file.to_owned(),
        }
    }
}

impl TryFrom<PathBuf> for TimestampedFile {
    type Error = TimelapsifyError;

    fn try_from(file: PathBuf) -> Result<Self, Self::Error> {
        let re = Lazy::force(&RE);

        let file_name = file
            .file_name()
            .ok_or_else(|| TimelapsifyError::file_option_issue(&file))?;

        let file_name_str = file_name.to_string_lossy();

        let dt = if let Some(cap) = re.captures(&file_name_str) {
            let year = cap
                .name("y")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            let month = cap
                .name("m")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            let day = cap
                .name("d")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            let hour = cap
                .name("H")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            let minutes = cap
                .name("M")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            let seconds = cap
                .name("S")
                .ok_or(TimelapsifyError::RegexNoMatch)?
                .as_str()
                .parse()?;
            Ok(Local.ymd(year, month, day).and_hms(hour, minutes, seconds))
        } else {
            Err(TimelapsifyError::RegexNoMatch)
        }?;

        Ok(Self {
            file,
            timestamp: dt,
        })
    }
}

/// Look at a folder containing images.
/// These are candidates for making a timelapse.
///
/// Async because this will run in a worker on the web server,
/// and we don't want to be blocking threads.
#[instrument(skip(folder), fields(dir = ?folder.as_ref()))]
async fn candidates<P: AsRef<Path>>(folder: P) -> Vec<TimestampedFile> {
    debug!("Looking for candidates in folder {:?}", folder.as_ref());
    let mut dir_stream = fs::read_dir(folder)
        .await
        .expect("should be able to read given folder");

    let mut candidates = vec![];

    while let Ok(Some(entry)) = dir_stream.next_entry().await {
        let file_metadata = entry
            .metadata()
            .await
            .expect("should be able to get file metadata");

        if !file_metadata.is_file() {
            continue;
        }

        if let Ok(timestamped_file) = entry.path().try_into() {
            candidates.push(timestamped_file);
        }
    }

    debug!("Candidates: {}", candidates.len());
    candidates
}

async fn move_all<P1: AsRef<Path>, P2: AsRef<Path>>(images: &[P1], output_folder: P2) {
    for image in images {
        trace!(
            "Moving {:?} to {:?}",
            image.as_ref(),
            output_folder.as_ref()
        );

        let out_folder = output_folder.as_ref().to_str().expect("out folder ok");

        let out = format!(
            "{out_folder}/{}",
            image
                .as_ref()
                .file_name()
                .expect("should be able to get file name")
                .to_str()
                .expect("&str ok")
        );

        fs::rename(&image, out)
            .await
            .expect("should be able to move to processed folder");
    }
}

/// Count the number of files with any of the given extensions in the given folder.
#[instrument(skip(folder), fields(dir = ?folder.as_ref()))]
pub async fn files_of_ext_in<P: AsRef<Path>>(folder: P, exts: &[&'static str]) -> Vec<PathBuf> {
    debug!("Looking for {:?} in {:?}", exts, folder.as_ref());
    let mut dir_stream = fs::read_dir(folder)
        .await
        .expect("should be able to read given folder");

    let mut files = vec![];

    while let Ok(Some(entry)) = dir_stream.next_entry().await {
        let file_metadata = entry
            .metadata()
            .await
            .expect("should be able to get file metadata");

        if !file_metadata.is_file() {
            continue;
        }

        let path = entry.path();
        let ext = match path.extension() {
            Some(e) => e,
            None => continue,
        };

        let ext = ext.to_str().expect("&str ok").to_ascii_lowercase();
        trace!("Comparing {ext} to {exts:?}");

        if !exts.iter().any(|&e| e == ext.as_str()) {
            continue;
        }

        files.push(entry.path());
    }

    debug!("Found {}", files.len());
    files
}

#[cfg(test)]
mod tests {
    use tracing::metadata::LevelFilter;

    use super::*;

    #[test]
    fn test_time_until_tomorrow() {
        let secs = duration_until_tomorrow_night_01().as_secs_f32();

        let hrs = secs / 60.0 / 60.0;
        dbg!(hrs);
    }

    #[tokio::test]
    async fn test_make_timelapses() {
        tracing_subscriber::fmt()
            .with_max_level(LevelFilter::DEBUG)
            .init();

        let images_unprocessed_path = format!("{}/test_images", env!("CARGO_MANIFEST_DIR"));
        let images_processed_path = format!("{}/test_images_processed", env!("CARGO_MANIFEST_DIR"));

        // For test purposes move any images back
        let images_processed = files_of_ext_in(&images_processed_path, &["jpg"]).await;
        move_all(&images_processed, &images_unprocessed_path).await;

        let mut candidates = candidates(images_unprocessed_path).await;

        sort_files_by_timestamp(&mut candidates);

        let before_day = images_before_day_of(
            candidates.clone(),
            Local.ymd(2022, 07, 12).and_hms(05, 30, 25),
        );
        let groups = group_by_day(before_day);
        // No images in test folder from before 2022-07-12
        assert_eq!(groups.keys().len(), 0);

        let before_day = images_before_day_of(
            candidates.clone(),
            Local.ymd(2022, 07, 13).and_hms(10, 50, 50),
        );
        let groups = group_by_day(before_day);
        // One group: 2022-07-12
        assert_eq!(groups.keys().len(), 1);

        let before_day = images_before_day_of(
            candidates.clone(),
            Local.ymd(2022, 07, 14).and_hms(00, 00, 00),
        );
        let groups = group_by_day(before_day);
        // Two groups: 2022-07-12, 2022-07-13
        assert_eq!(groups.keys().len(), 2);

        let before_day = images_before_day_of(
            candidates.clone(),
            Local.ymd(2025, 01, 01).and_hms(00, 00, 00),
        );
        let groups = group_by_day(before_day);
        // Two groups: 2022-07-12, 2022-07-13
        assert_eq!(groups.keys().len(), 2);

        for (dt, images) in groups {
            let output = ffmpeg_make_clip(
                &images,
                &format!("output_mp4/days/{}.mp4", dt.format("%Y-%m-%d")),
            )
            .await;

            println!("{output:?}");
            assert!(output.status.success());
            move_all(&images, &images_processed_path).await;
        }

        let videos = files_of_ext_in("output_mp4/days", &["mp4"]).await;
        assert_eq!(2, videos.len());
    }
}
