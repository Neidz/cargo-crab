use image::{ Rgba};

use crate::{image_io::ImageIO, pixel_art_scanner::PixelArt};

mod pixel_art_scanner;
mod image_io;

fn main() {
    let target_image = ImageIO::load_rgba_image("assets/images/crewmate.png").unwrap();
    let source_image = ImageIO::load_rgba_image("assets/images/final_2023_place.png").unwrap();

    let searched_color = Rgba([0,0,0, 1]);
    let searching_tolerance = 1;

    let target_pixel_art = PixelArt::new(target_image, &searched_color, searching_tolerance).unwrap();
}
