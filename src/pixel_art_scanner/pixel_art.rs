use core::fmt;
use std::{collections::HashSet};

use image::{ Rgba, RgbaImage};
use anyhow::{anyhow, Result};

use super::color_utils::ColorUtils;

pub struct PixelArt {
    image: RgbaImage,
    coordinates: Vec<(u32, u32)>,
    coordinates_of_adjacent_pixels: Vec<(i32, i32)>
}

#[derive(Debug)]
pub enum PixelArtError {
    EmptyCoordinates,

}

impl fmt::Display for PixelArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelArtError::EmptyCoordinates => write!(f, "Failed to extract any coordinates with that specified color")
        }
    }
}

impl std::error::Error for PixelArtError {}

impl PixelArt {
    pub fn new(image: RgbaImage, searched_color: &Rgba<u8>, searching_tolerance: u8) -> Result<Self> {
        let coordinates = PixelArt::get_coordinates(&image, searched_color, searching_tolerance);
        let coordinates_of_adjacent_pixels = PixelArt::get_coordinates_of_adjacent_pixels(&coordinates);

        if coordinates.len() == 0 {
            return Err(anyhow!(PixelArtError::EmptyCoordinates));
        }

        Ok(PixelArt { image, coordinates, coordinates_of_adjacent_pixels })
    }

    fn get_coordinates(image: &RgbaImage, searched_color: &Rgba<u8>, searching_tolerance: u8) -> Vec<(u32, u32)> {
        let (img_width, img_height) = image.dimensions();

        let mut coordinates: Vec<(u32, u32)> = vec![];

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel_color = image.get_pixel(x, y);

                let equal = ColorUtils::equal_with_tolerance(pixel_color, searched_color, searching_tolerance);

                if equal {
                    coordinates.push((x,y));
                }                
            }
        }


        coordinates
    }

    fn get_coordinates_of_adjacent_pixels(coordinates: &Vec<(u32, u32)>) -> Vec<(i32, i32)> {
        let mut adjacent_coordinates = HashSet::new();

        for (x,y) in coordinates {
            for (offset_x, offset_y) in SURROUNDING_OFFSETS {
                let x_with_offset = *x as i32 + offset_x;
                let y_with_offset = *y as i32 + offset_y;

                let new_coord = (x_with_offset, y_with_offset);

                if new_coord.0 < 0 || new_coord.1 < 0 {
                    adjacent_coordinates.insert(new_coord);
                    continue;
                }

                if !coordinates.contains(&(new_coord.0 as u32, new_coord.1 as u32)) {
                    adjacent_coordinates.insert(new_coord);
                }
            }
        }

        adjacent_coordinates.into_iter().collect()
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        let mut highest_x = 0;
        let mut highest_y = 0;

        for &(x,y) in &self.coordinates {
            if x > highest_x {
                highest_x = x;
            }
            if y > highest_y {
                highest_y = y;
            }
        }
        (highest_x +1, highest_y + 1)
    }
}

const SURROUNDING_OFFSETS: [(i32, i32); 8] = [
    (LEFT, TOP),
    (CENTER, TOP),
    (RIGHT, TOP),
    (LEFT, CENTER),
    (RIGHT, CENTER),
    (LEFT, BOTTOM),
    (CENTER, BOTTOM),
    (RIGHT, BOTTOM),
];

const LEFT: i32 = -1;
const RIGHT: i32 = 1;
const TOP: i32 = -1;
const BOTTOM: i32 = 1;
const CENTER: i32 = 0;