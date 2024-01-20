use core::fmt;
use std::collections::HashSet;

use anyhow::{anyhow, Result};
use image::{Rgba, RgbaImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use super::{color_utils::ColorUtils, config::Config};

pub struct PixelArt {
    config: Config,
    coordinates: Vec<(u32, u32)>,
    coordinates_of_adjacent_pixels: Vec<(i32, i32)>,
}

#[derive(Debug)]
pub enum PixelArtError {
    EmptyCoordinates,
}

impl fmt::Display for PixelArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PixelArtError::EmptyCoordinates => write!(
                f,
                "Failed to extract any coordinates with that specified color"
            ),
        }
    }
}

impl std::error::Error for PixelArtError {}

impl PixelArt {
    pub fn new(image: RgbaImage, config: Config) -> Result<Self> {
        let coordinates =
            PixelArt::get_coordinates(&image, &config.searched_color, &config.extracting_tolerance);
        let coordinates_of_adjacent_pixels =
            PixelArt::get_coordinates_of_adjacent_pixels(&coordinates);

        if coordinates.is_empty() {
            return Err(anyhow!(PixelArtError::EmptyCoordinates));
        }

        Ok(PixelArt {
            coordinates,
            coordinates_of_adjacent_pixels,
            config,
        })
    }

    fn get_coordinates(
        image: &RgbaImage,
        searched_color: &Rgba<u8>,
        searching_tolerance: &u8,
    ) -> Vec<(u32, u32)> {
        let (img_width, img_height) = image.dimensions();

        let mut coordinates: Vec<(u32, u32)> = vec![];

        for y in 0..img_height {
            for x in 0..img_width {
                let pixel_color = image.get_pixel(x, y);

                let equal = ColorUtils::equal_with_tolerance(
                    pixel_color,
                    searched_color,
                    *searching_tolerance,
                );

                if equal {
                    coordinates.push((x, y));
                }
            }
        }

        coordinates
    }

    fn get_coordinates_of_adjacent_pixels(coordinates: &Vec<(u32, u32)>) -> Vec<(i32, i32)> {
        let mut adjacent_coordinates = HashSet::new();

        for (x, y) in coordinates {
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

    pub fn search_in_image(&self, searched_image: &RgbaImage) -> Vec<Vec<(u32, u32)>> {
        let (img_width, img_height) = searched_image.dimensions();
        let (window_width, window_height) = self.get_window_size();

        let found_instances: Vec<Vec<(u32, u32)>> = (0..(img_height - window_height))
            .into_par_iter()
            .flat_map(|offset_y| {
                (0..(img_width - window_width))
                    .into_par_iter()
                    .filter_map(move |offset_x| {
                        self.pixel_art_instance_in_window(offset_x, offset_y, searched_image)
                    })
            })
            .collect();

        found_instances
    }

    fn pixel_art_instance_in_window(
        &self,
        offset_x: u32,
        offset_y: u32,
        searched_image: &RgbaImage,
    ) -> Option<Vec<(u32, u32)>> {
        let coordinates_with_offset: Vec<(u32, u32)> = self
            .coordinates
            .iter()
            .map(|&(x, y)| (x + offset_x, y + offset_y))
            .collect();

        let first_pixel_color =
            searched_image.get_pixel(coordinates_with_offset[0].0, coordinates_with_offset[0].1);

        for &(x, y) in &coordinates_with_offset {
            let pixel_color = searched_image.get_pixel(x, y);

            if !ColorUtils::equal_with_tolerance(
                first_pixel_color,
                pixel_color,
                self.config.searching_similarity_tolerance,
            ) {
                return None;
            }
        }

        let coordinates_of_adjacent_pixels_with_offset: Vec<(i32, i32)> = self
            .coordinates_of_adjacent_pixels
            .iter()
            .map(|&(x, y)| (x + offset_x as i32, y + offset_y as i32))
            .collect();

        for (x, y) in coordinates_of_adjacent_pixels_with_offset {
            if x < 0 || y < 0 {
                continue;
            }

            let adjacent_pixel_color = searched_image.get_pixel_checked(x as u32, y as u32);

            if let Some(adjacent_pixel_color) = adjacent_pixel_color {
                if ColorUtils::equal_with_tolerance(
                    first_pixel_color,
                    adjacent_pixel_color,
                    self.config.searching_contrast_tolerance,
                ) {
                    return None;
                }
            }
        }

        Some(coordinates_with_offset)
    }

    fn get_window_size(&self) -> (u32, u32) {
        let mut highest_x = 0;
        let mut highest_y = 0;

        for &(x, y) in &self.coordinates {
            if x > highest_x {
                highest_x = x;
            }
            if y > highest_y {
                highest_y = y;
            }
        }
        (highest_x + 1, highest_y + 1)
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
