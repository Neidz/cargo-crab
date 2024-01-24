use std::{fs::create_dir_all, path::PathBuf};

use image::{io::Reader as ImageReader, RgbImage};

pub struct ImageIO {}

use anyhow::Result;

impl ImageIO {
    pub fn load_rgb_image(path: &PathBuf) -> Result<RgbImage> {
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

    pub fn save_image(image: &RgbImage, path: &str, name: &str, extension: &str) -> Result<()> {
        let directory_path = PathBuf::from(path);

        if !directory_path.exists() {
            create_dir_all(&directory_path)?;
        }

        let mut path_to_file = directory_path;

        path_to_file.push(format!("{}{}", name, extension));

        image.save(path_to_file)?;

        Ok(())
    }
}
