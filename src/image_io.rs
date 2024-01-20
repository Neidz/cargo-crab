use std::path::Path;

use image::{io::Reader as ImageReader, RgbaImage};

pub struct ImageIO {}

use anyhow::Result;

impl ImageIO {
    pub fn load_rgba_image(path: &str) -> Result<RgbaImage> {
        let img = ImageReader::open(Path::new(path))?.decode()?;

        let rgba_img: RgbaImage = img.to_rgba8();

        Ok(rgba_img)
    }

    pub fn load_multiple_rgba_images(paths: Vec<&str>) -> Result<Vec<RgbaImage>> {
        paths
            .iter()
            .map(|path| ImageIO::load_rgba_image(path))
            .collect()
    }
}
