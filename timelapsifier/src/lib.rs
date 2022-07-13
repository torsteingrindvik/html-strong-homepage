use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local, TimeZone, Timelike};
use regex::Regex;
use tokio::{fs, io::AsyncWriteExt, process::Command, sync::OnceCell};

#[derive(Debug, Clone)]
pub struct StoredImage {
    pub file: PathBuf,
    pub timestamp: DateTime<Local>,
}

static RE: OnceCell<Regex> = OnceCell::const_new();

pub async fn ffmpeg_make_clip(images: &[StoredImage], output: &str) {
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

    let output = Command::new("ffmpeg")
        .args([
            "-y", "-safe", "0", "-f", "concat", "-r", "60", "-i", file_name, "-b:v", "1M", output,
        ])
        .output()
        .await
        .expect("Couldn't run ffmpeg properly");

    dbg!(output);
}

/// Sort images such that earlier timestamped images
/// come first.
pub fn sort_images(images: &mut [StoredImage]) {
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
pub fn images_before_day_of(
    images: Vec<StoredImage>,
    timestamp: DateTime<Local>,
) -> Vec<StoredImage> {
    let timestamp = floor_to_day(&timestamp);

    images
        .into_iter()
        .take_while(|image| image.timestamp < timestamp)
        .collect()
}

/// Group a vector of sorted images into groups by the day.
pub fn group_by_day(images: Vec<StoredImage>) -> BTreeMap<DateTime<Local>, Vec<StoredImage>> {
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
pub async fn candidates<P: AsRef<Path>>(folder: P) -> Vec<StoredImage> {
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

#[cfg(test)]
mod tests {
    use chrono::Datelike;

    use super::*;

    #[tokio::test]
    async fn test_add() {
        let mut candidates =
            candidates(format!("{}/test_images", env!("CARGO_MANIFEST_DIR"))).await;

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
            ffmpeg_make_clip(&images, &format!("day-{}.webm", dt.day())).await;
        }
    }
}
