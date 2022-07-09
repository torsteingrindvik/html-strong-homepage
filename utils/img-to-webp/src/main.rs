use std::{path::PathBuf, process::Command};

use color_eyre::Result;
use walkdir::WalkDir;

fn is_img(path: &PathBuf) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("png".into())
        || path.extension().and_then(|ext| ext.to_str()) == Some("jpg".into())
}

fn webp_does_not_exist(path: &PathBuf) -> bool {
    !path.with_extension("webp").exists()
}

fn main() -> Result<()> {
    // Create thumbnails if they don't exist
    for (img_in, thumb_webp_out) in WalkDir::new(".") // Recurse all files starting from current dir
        .into_iter()
        // Ignore any problematic entries
        .filter_map(|e| e.ok())
        // We want to look at paths
        .map(|f| f.path().to_owned())
        // Only care about png, jpg files
        .filter(is_img)
        .map(|f| {
            (
                f.clone(),
                format!(
                    "{}-thumbnail.webp",
                    f.file_stem()
                        .expect("file name should exist")
                        .to_str()
                        .expect("file name should be valid &str")
                ),
            )
        })
        // .inspect(|bah| {dbg!(&bah);})
        .filter(|(f, thumb_name)| !f.with_file_name(thumb_name).exists())
        .map(|(f, thumb_name)| (f.clone(), f.with_file_name(thumb_name)))
    {
        println!("Making thumbnail for {img_in:?}, output is {thumb_webp_out:?}");

        let img = image::open(&img_in).expect("should be able to open image");
        let img = img.resize(1280, 720, image::imageops::FilterType::Lanczos3);

        let width = img.width();
        let height = img.height();

        println!("Output size: {width:?} {height:?}");

        let huh = Command::new("cwebp")
            .args([
                &format!("-resize {width} {height}"),
                // Multithreading
                "-mt",
                img_in.to_str().expect("valid &str"),
                "-o".into(),
                &thumb_webp_out.to_str().expect("valid &str"),
            ])
            .output()?;
        dbg!(huh);
        std::process::exit(0);
    }

    for (img_in, webp_out) in WalkDir::new(".") // Recurse all files starting from current dir
        .into_iter()
        // Ignore any problematic entries
        .filter_map(|e| e.ok())
        // We want to look at paths
        .map(|f| f.path().to_owned())
        // Only care about png, jpg files
        .filter(is_img)
        // If it's already converted, don't do it again
        .filter(webp_does_not_exist)
        .map(|f| (f.clone(), f.with_extension("webp")))
    {
        println!("Converting {img_in:?}");
        Command::new("cwebp")
            .args([
                // Multithreading
                "-mt",
                img_in.to_str().expect("valid &str"),
                "-o".into(),
                webp_out.to_str().expect("valid &str"),
            ])
            .output()?;
    }

    Ok(())
}
