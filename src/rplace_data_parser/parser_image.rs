use image::RgbImage;

use super::record::{Coordinate, Record};

pub struct ImageExpansionOffset {
    left: i32,
    top: i32,
}

pub struct ParserImage {
    image: RgbImage,
    image_expansion_offset: ImageExpansionOffset,
}

impl ParserImage {
    pub fn new() -> ParserImage {
        ParserImage {
            image: RgbImage::new(0, 0),
            image_expansion_offset: ImageExpansionOffset { left: 0, top: 0 },
        }
    }

    pub fn save_image(&self, seconds_passed: u32) {
        todo!()
    }

    pub fn handle_record(&self, record: &Record) {
        self.draw_from_record(record);
        self.handle_image_expansion(&record.coordinate);
    }

    fn draw_from_record(&self, record: &Record) {
        match record.coordinate {
            Coordinate::Point { x, y } => {
                todo!()
            }
            Coordinate::Rectangle { x1, y1, x2, y2 } => {
                todo!()
            }
            Coordinate::Circle { x, y, r } => {
                todo!()
            }
        }
    }

    fn handle_image_expansion(&self, coordinate: &Coordinate) {
        let (img_width, img_height) = self.image.dimensions();
        let (img_width, img_height) = (img_width as i32, img_height as i32);

        let mut expand_left = 0;
        let mut expand_right = 0;
        let mut expand_top = 0;
        let mut expand_bottom = 0;

        match coordinate {
            Coordinate::Point { x, y } => {
                let x_with_offset = x + self.image_expansion_offset.left;
                let y_with_offset = y + self.image_expansion_offset.top;

                if x_with_offset < 0 {
                    expand_left = -x_with_offset;
                }
                if x_with_offset >= img_width {
                    expand_right = x_with_offset - img_width + 1;
                }
                if y_with_offset < 0 {
                    expand_top = -y_with_offset;
                }
                if y_with_offset >= img_height {
                    expand_bottom = y_with_offset - img_height + 1;
                }
            }
            Coordinate::Rectangle { x1, y1, x2, y2 } => {
                todo!()
            }
            Coordinate::Circle { x, y, r } => {
                todo!()
            }
        }
    }
}
