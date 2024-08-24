use anyhow::Result;
use image::{ImageFormat, ImageReader};
use std::fs::create_dir_all;
use std::path::Path;

pub fn save_as_png(image_path: &Path, output_path: &Path) -> Result<()> {
    if let Some(output_dir) = output_path.parent() {
        create_dir_all(output_dir)?;
    }
    let img = ImageReader::open(image_path)?.decode()?;
    img.save_with_format(output_path, ImageFormat::Png)?;

    Ok(())
}
