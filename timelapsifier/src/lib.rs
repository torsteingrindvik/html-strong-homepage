use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};

use chrono::{DateTime, Local, TimeZone, Timelike};
use regex::Regex;
use tokio::{
    fs,
    io::AsyncWriteExt,
    process::Command,
    sync::{OnceCell, RwLock},
};
use tracing::{debug, error, info, trace};

#[derive(Debug, Clone)]
struct StoredImage {
    pub file: PathBuf,
    pub timestamp: DateTime<Local>,
}

impl AsRef<Path> for StoredImage {
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

pub type StateWebms = Arc<RwLock<Vec<PathBuf>>>;

pub struct TimelapserOptions {
    pub unprocessed_images_folder: PathBuf,
    pub processed_images_folder: PathBuf,
    pub timelapse_output_folder: PathBuf,

    pub timelapse_webms: StateWebms,
}

async fn do_work(options: &TimelapserOptions) {
    info!("Looking for timelapse image candidates");

    let mut candidates = candidates(&options.unprocessed_images_folder).await;
    if candidates.is_empty() {
        info!("No candidates");
        return;
    }
    sort_images(&mut candidates);

    info!("Found {} candidates", candidates.len());

    let before_today = images_before_day_of(candidates.clone(), Local::now());
    if before_today.is_empty() {
        info!("No images from earlier days");
        return;
    }

    let grouped_by_days = group_by_day(before_today);

    for (day, images_that_day) in grouped_by_days {
        info!("Making a timelapse for day {day:?}");

        let mut output_webm = options.timelapse_output_folder.clone();
        if !output_webm.exists() {
            info!("Making output dir {output_webm:?}");
            fs::create_dir_all(&output_webm)
                .await
                .expect("make dirs ok");
        }

        output_webm.push(format!("{}.webm", day.format("%Y-%m-%d")));

        let output = ffmpeg_make_clip(
            &images_that_day,
            output_webm.to_str().expect("ok &str webm path"),
        )
        .await;

        if !output.status.success() {
            error!(?output, "Timelapse creation not successful!");
            return;
        } else {
            info!("Timelapse made ok: {output_webm:?}")
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

        let webms = files_of_ext_in(&options.timelapse_output_folder, &["webm"]).await;
        info!("# webms: {}", webms.len());
        *options.timelapse_webms.write().await = webms;

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

static RE: OnceCell<Regex> = OnceCell::const_new();

async fn ffmpeg_make_clip(images: &[StoredImage], output_webm: &str) -> Output {
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
            "-y",
            "-safe",
            "0",
            "-f",
            "concat",
            "-r",
            "60",
            "-i",
            file_name,
            "-b:v",
            "1M",
            output_webm,
        ])
        .output()
        .await
        .expect("Couldn't run ffmpeg properly")
}

/// Sort images such that earlier timestamped images
/// come first.
fn sort_images(images: &mut [StoredImage]) {
    images.sort_unstable_by_key(|image| image.timestamp);
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
fn images_before_day_of(images: Vec<StoredImage>, timestamp: DateTime<Local>) -> Vec<StoredImage> {
    let timestamp = floor_to_day(&timestamp);

    images
        .into_iter()
        .take_while(|image| image.timestamp < timestamp)
        .collect()
}

/// Group a vector of sorted images into groups by the day.
fn group_by_day(images: Vec<StoredImage>) -> BTreeMap<DateTime<Local>, Vec<StoredImage>> {
    let mut groups = BTreeMap::<DateTime<Local>, Vec<StoredImage>>::new();

    for image in images {
        let t = floor_to_day(&image.timestamp);
        groups.entry(t).or_default().push(image);
    }

    groups
}

/// Look at a folder containing images.
/// These are candidates for making a timelapse.
///
/// Async because this will run in a worker on the web server,
/// and we don't want to be blocking threads.
async fn candidates<P: AsRef<Path>>(folder: P) -> Vec<StoredImage> {
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

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Example:
        //  2022-07-13_17-09-23.jpg
        let re = RE
            .get_or_init(|| async {
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
            })
            .await;

        let dt = if let Some(cap) = re.captures(&file_name_str) {
            let year = cap
                .name("y")
                .expect("year should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");
            let month = cap
                .name("m")
                .expect("month should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");
            let day = cap
                .name("d")
                .expect("day should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");
            let hour = cap
                .name("H")
                .expect("hour should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");
            let minutes = cap
                .name("M")
                .expect("minutes should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");
            let seconds = cap
                .name("S")
                .expect("seconds should have been captured")
                .as_str()
                .parse()
                .expect("parse ok");

            Local.ymd(year, month, day).and_hms(hour, minutes, seconds)
        } else {
            continue;
        };

        candidates.push(StoredImage {
            file: entry.path(),
            timestamp: dt,
        });
    }

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
        tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

        let images_unprocessed_path = format!("{}/test_images", env!("CARGO_MANIFEST_DIR"));
        let images_processed_path = format!("{}/test_images_processed", env!("CARGO_MANIFEST_DIR"));

        // For test purposes move any images back
        let images_processed = files_of_ext_in(&images_processed_path, &["jpg"]).await;
        move_all(&images_processed, &images_unprocessed_path).await;

        let mut candidates = candidates(images_unprocessed_path).await;

        sort_images(&mut candidates);

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
                &format!("output_webm/days/{}.webm", dt.format("%Y-%m-%d")),
            )
            .await;

            println!("{output:?}");
            assert!(output.status.success());
            move_all(&images, &images_processed_path).await;
        }

        let webms = files_of_ext_in("output_webm/days", &["webm"]).await;
        assert_eq!(2, webms.len());
    }
}
