use std::path::PathBuf;

use anyhow::Result;
use chrono::NaiveDateTime;
use csv::Reader;

use super::{
    config::{OnError, ParserConfig},
    parser_image::ParserImage,
    record::Record,
};

pub struct Parser {
    config: ParserConfig,
    parser_image: ParserImage,
}

impl Parser {
    pub fn new(config: ParserConfig) -> Parser {
        Parser {
            config,
            parser_image: ParserImage::new(),
        }
    }

    pub fn parse(&self, paths: &[PathBuf]) -> Result<()> {
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

                self.parser_image.handle_record(&record);

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

                            self.parser_image.save_image(elapsed_seconds);
                        }
                    }
                };
            }
        }

        Ok(())
    }
}
