use std::path::{Path, PathBuf};

use image::{io::Reader as ImageReader, RgbImage};

pub struct ImageIO {}

use anyhow::Result;

impl ImageIO {
    pub fn load_rgb_image(path: &Path) -> Result<RgbImage> {
        let img = ImageReader::open(path)?.decode()?;

        let rgb_img: RgbImage = img.to_rgb8();

        Ok(rgb_img)
    }

    pub fn load_multiple_rgb_images(paths: &[PathBuf]) -> Result<Vec<RgbImage>> {
        paths
            .iter()
            .map(|path| ImageIO::load_rgb_image(path))
            .collect()
    }
}
