use std::process::Command;

use color_eyre::Result;
use walkdir::WalkDir;

fn main() -> Result<()> {
    for (png_in, webp_out) in WalkDir::new(".") // Recurse all files starting from current dir
        .into_iter()
        // Ignore any problematic entries
        .filter_map(|e| e.ok())
        // We want to look at paths
        .map(|f| f.path().to_owned())
        // Only care about png files
        .filter(|f| f.extension().and_then(|ext| ext.to_str()) == Some("png".into()))
        // If it's already converted, don't do it again
        .filter(|f| !f.with_extension("webp").exists())
        .map(|f| (f.clone(), f.with_extension("webp")))
    {
        println!("Converting {png_in:?}");
        Command::new("cwebp")
            .args([png_in, "-o".into(), webp_out])
            .output()?;
    }

    Ok(())
}
