use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(with = "timestamp_format")]
    pub timestamp: NaiveDateTime,
    pub user: String,
    pub coordinate: String,
    pub pixel_color: String,
}

mod timestamp_format {
    use anyhow::Result;
    use chrono::NaiveDateTime;
    use serde::{Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f %Z";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
