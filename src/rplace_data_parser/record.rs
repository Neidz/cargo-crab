use anyhow::Result;
use chrono::NaiveDateTime;
use image::Rgb;
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum Coordinate {
    Point { x: i32, y: i32 },
    Rectangle { x1: i32, y1: i32, x2: i32, y2: i32 },
    Circle { x: i32, y: i32, r: u32 },
}

#[derive(Debug, Deserialize)]
pub struct Record {
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: NaiveDateTime,
    pub user: String,
    #[serde(deserialize_with = "deserialize_coordinate")]
    pub coordinate: Coordinate,
    #[serde(deserialize_with = "deserialize_color")]
    pub pixel_color: Rgb<u8>,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f %Z").map_err(serde::de::Error::custom)
}

fn deserialize_coordinate<'de, D>(deserializer: D) -> Result<Coordinate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let filtered_s: String = s
        .chars()
        .filter(|&c| c.is_digit(10) || c == ',' || c == '-')
        .collect();

    let numbers: Vec<&str> = filtered_s.split(',').collect();

    match numbers.len() {
        2 => {
            let x = numbers[0]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let y = numbers[1]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;

            Ok(Coordinate::Point { x, y })
        }
        3 => {
            let x = numbers[0]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let y = numbers[1]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let r = numbers[2]
                .parse::<u32>()
                .map_err(serde::de::Error::custom)?;

            Ok(Coordinate::Circle { x, y, r })
        }
        4 => {
            let x1 = numbers[0]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let y1 = numbers[1]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let x2 = numbers[2]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;
            let y2 = numbers[3]
                .parse::<i32>()
                .map_err(serde::de::Error::custom)?;

            Ok(Coordinate::Rectangle { x1, y1, x2, y2 })
        }
        _ => Err(serde::de::Error::custom(format!(
            "Failed to parse coordinate, expected (x,y), (x,y,r) or (x1,y1,x2,y2) but got {:?}",
            s
        ))),
    }
}

fn deserialize_color<'de, D>(deserializer: D) -> Result<Rgb<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let r = u8::from_str_radix(&s[1..3], 16).map_err(serde::de::Error::custom)?;
    let g = u8::from_str_radix(&s[3..5], 16).map_err(serde::de::Error::custom)?;
    let b = u8::from_str_radix(&s[5..7], 16).map_err(serde::de::Error::custom)?;

    Ok(Rgb([r, g, b]))
}
