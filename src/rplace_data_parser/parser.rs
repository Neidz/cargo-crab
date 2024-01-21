use std::path::PathBuf;

use anyhow::Result;
use chrono::NaiveDateTime;
use csv::Reader;
use image::RgbaImage;

use super::{
    config::{OnError, ParserConfig},
    record::Record,
};

struct ImageExpansionOffset {
    pub left: u32,
    pub top: u32,
}

pub struct Parser {
    config: ParserConfig,
    image: RgbaImage,
    image_expansion_offset: ImageExpansionOffset,
}

impl Parser {
    pub fn new(config: ParserConfig) -> Parser {
        Parser {
            config,
            image: RgbaImage::new(0, 0),
            image_expansion_offset: ImageExpansionOffset { left: 0, top: 0 },
        }
    }

    pub fn parse(&self, paths: &Vec<PathBuf>) -> Result<()> {
        let mut first_timestamp: Option<NaiveDateTime> = None;
        let mut last_action: u32 = 0;

        for path in paths {
            let mut reader = Reader::from_path(path)?;

            for result in reader.deserialize() {
                let record: Record = match result {
                    Ok(record) => record,
                    Err(err) => match self.config.on_error {
                        OnError::Nothing => continue,
                        OnError::Print => {
                            eprintln!("Error parsing record: {}. Skipping", err);
                            continue;
                        }
                        OnError::Stop => return Err(err.into()),
                    },
                };

                self.draw_pixel(&record);

                match first_timestamp {
                    None => {
                        first_timestamp = Some(record.timestamp);
                    }
                    Some(first_timestamp) => {
                        let elapsed_time = record.timestamp - first_timestamp;
                        let elapsed_seconds = elapsed_time.num_seconds() as u32;

                        if elapsed_seconds > (last_action + self.config.save_interval_seconds) {
                            let elapsed_intervals =
                                (elapsed_seconds - last_action) / self.config.save_interval_seconds;

                            last_action += elapsed_intervals * self.config.save_interval_seconds;

                            self.save_image(elapsed_seconds);
                        }
                    }
                };
            }
        }

        Ok(())
    }

    fn save_image(&self, seconds_passed: u32) {
        println!("{:?}", seconds_passed);
    }

    fn draw_pixel(&self, record: &Record) {}
}
