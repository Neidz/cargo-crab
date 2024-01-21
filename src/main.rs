use std::{path::PathBuf, time::Instant};

use pixel_art_scanner::Config;
use rplace_data_parser::{Parser, ParserConfig};

use crate::{image_io::ImageIO, pixel_art_scanner::PixelArt};

mod image_io;
mod pixel_art_scanner;
mod rplace_data_parser;

fn main() {
    let start_time = Instant::now();

    test_parser();

    let end_time = Instant::now();
    let elapsed_time = end_time - start_time;
    println!("Elapsed time: {:.2?}", elapsed_time);
}

fn test_scan_image() {
    let target_image = ImageIO::load_rgba_image("assets/images/crewmate.png").unwrap();
    let source_image = ImageIO::load_rgba_image("assets/images/final_2023_place.png").unwrap();

    let target_pixel_art = PixelArt::new(target_image, Config::new_default()).unwrap();

    println!(
        "{:?}",
        target_pixel_art.search_in_image(&source_image).len()
    );
}

fn test_parser() {
    let paths = vec![PathBuf::from(
        "assets/rplace_data_sample/2023_place_canvas_history-000000000000.csv",
    )];
    let parser = Parser::new(ParserConfig::new_default());

    parser.parse(&paths).unwrap();
}
