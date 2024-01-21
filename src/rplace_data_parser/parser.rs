use anyhow::Result;
use csv::Reader;

use super::record::Record;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self) -> Result<()> {
        let mut reader =
            Reader::from_path("assets/rplace_data_sample/different_forms_of_coordinates.csv")?;

        for result in reader.deserialize() {
            let record: Record = result?;

            println!("{:?}", record);
        }

        Ok(())
    }
}
