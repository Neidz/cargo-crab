use std::{path::PathBuf, time::Instant};

use image::Rgb;
use pixel_art_scanner::Config;
use rplace_data_parser::{Parser, ParserConfig};

use crate::{image_io::ImageIO, pixel_art_scanner::PixelArt};

mod image_io;
mod pixel_art_scanner;
mod rplace_data_parser;

fn main() {
    let start_time = Instant::now();

    // test_parser();
    test_scan_image();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:.2?}", elapsed_time);
}

fn test_scan_image() {
    let target_image =
        ImageIO::load_rgb_image(&PathBuf::from("assets/images/crewmate.png")).unwrap();
    let source_image =
        ImageIO::load_rgb_image(&PathBuf::from("assets/images/final_2023_place.png")).unwrap();

    let target_pixel_art = PixelArt::new(target_image, Config::new_default()).unwrap();

    let found_instances = target_pixel_art.search_in_image(&source_image);

    let visualization = PixelArt::visualize_pixel_arts(
        &source_image,
        &found_instances,
        &Rgb([255, 255, 255]),
        &Rgb([0, 0, 0]),
    );

    ImageIO::save_image(
        &visualization,
        "output/visualization",
        "crewmate_visualization",
        ".png",
    )
    .unwrap();

    println!("{:?}", found_instances.len());
}

fn test_parser() {
    let mut paths = Vec::new();

    for i in 0..=52 {
        let filename = format!("assets/rplace_data/2023_place_canvas_history-{:012}.csv", i);
        paths.push(PathBuf::from(filename));
    }

    let mut parser = Parser::new(ParserConfig::new_default());

    parser.parse(&paths).unwrap();
}
