use image::Rgb;

pub struct Config {
    pub extracting_tolerance: u8,
    pub searching_similarity_tolerance: u8,
    pub searching_contrast_tolerance: u8,
    pub searched_color: Rgb<u8>,
}

impl Config {
    pub fn new(
        extracting_tolerance: u8,
        searching_similarity_tolerance: u8,
        searching_contrast_tolerance: u8,
        searched_color: Rgb<u8>,
    ) -> Config {
        Config {
            extracting_tolerance,
            searching_similarity_tolerance,
            searching_contrast_tolerance,
            searched_color,
        }
    }

    pub fn new_default() -> Config {
        Config {
            extracting_tolerance: 1,
            searching_similarity_tolerance: 1,
            searching_contrast_tolerance: 1,
            searched_color: Rgb([1, 1, 1]),
        }
    }
}
