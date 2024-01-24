use image::{imageops, ImageBuffer, Rgb, RgbImage};

use crate::image_io::ImageIO;

use super::record::{Coordinate, Record};

#[derive(Debug)]
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

    pub fn save_image(&self, output_dir: &str, seconds_passed: u32) {
        ImageIO::save_image(&self.image, output_dir, &seconds_passed.to_string(), ".png").unwrap()
    }

    pub fn handle_record(&mut self, record: &Record) {
        self.handle_image_expansion(&record.coordinate);
        self.draw_from_record(record);
    }

    fn draw_from_record(&mut self, record: &Record) {
        let ImageExpansionOffset { left, top } = self.image_expansion_offset;
        let offset_left = left;
        let offset_top = top;

        match record.coordinate {
            Coordinate::Point { x, y } => {
                let x_with_offset = x + offset_left;
                let y_with_offset = y + offset_top;

                self.image.put_pixel(
                    x_with_offset as u32,
                    y_with_offset as u32,
                    record.pixel_color,
                );
            }
            Coordinate::Rectangle { x1, y1, x2, y2 } => {
                let x1_with_offset = x1 + offset_left;
                let y1_with_offset = y1 + offset_top;
                let x2_with_offset = x2 + offset_left;
                let y2_with_offset = y2 + offset_top;

                for y in y1_with_offset..=(y2_with_offset) {
                    for x in x1_with_offset..=(x2_with_offset) {
                        self.image.put_pixel(x as u32, y as u32, record.pixel_color);
                    }
                }
            }
            Coordinate::Circle { x, y, r } => {
                let x_with_offset = x + offset_left;
                let y_with_offset = y + offset_top;
                let r = r as i32;

                for y in (x_with_offset - r)..=(x_with_offset + r) {
                    for x in (y_with_offset - r)..=(y_with_offset + r) {
                        let dx = x - x_with_offset;
                        let dy = y - y_with_offset;
                        if dx * dx + dy * dy <= r * r {
                            self.image.put_pixel(x as u32, y as u32, record.pixel_color);
                        }
                    }
                }
            }
        }
    }

    fn handle_image_expansion(&mut self, coordinate: &Coordinate) {
        let (img_width, img_height) = self.image.dimensions();
        let (img_width, img_height) = (img_width as i32, img_height as i32);
        let ImageExpansionOffset { left, top } = self.image_expansion_offset;
        let offset_left = left;
        let offset_top = top;

        let mut expand_left = 0;
        let mut expand_right = 0;
        let mut expand_top = 0;
        let mut expand_bottom = 0;

        match coordinate {
            Coordinate::Point { x, y } => {
                let x_with_offset = x + offset_left;
                let y_with_offset = y + offset_top;

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
                let x1_with_offset = x1 + offset_left;
                let y1_with_offset = y1 + offset_top;
                let x2_with_offset = x2 + offset_left;
                let y2_with_offset = y2 + offset_top;

                if x1_with_offset < 0 {
                    expand_left = -x1_with_offset;
                }
                if x2_with_offset >= img_width {
                    expand_right = x2_with_offset - img_width + 1;
                }
                if y1_with_offset < 0 {
                    expand_top = -y1_with_offset;
                }
                if y2_with_offset >= img_height {
                    expand_bottom = y2_with_offset - img_height + 1;
                }
            }
            Coordinate::Circle { x, y, r } => {
                let x_with_offset = x + offset_left;
                let y_with_offset = y + offset_top;
                let r = *r as i32;

                if x_with_offset - r < 0 {
                    expand_left = -x_with_offset + r;
                }
                if x_with_offset + r >= img_width {
                    expand_right = x_with_offset + r - img_width + 1;
                }
                if y_with_offset - r < 0 {
                    expand_top = -y_with_offset + r;
                }
                if y_with_offset + r >= img_height {
                    expand_bottom = y_with_offset + r - img_height + 1;
                }
            }
        };

        if expand_left != 0 || expand_right != 0 || expand_top != 0 || expand_bottom != 0 {
            let new_height = img_height + expand_top + expand_bottom;
            let new_width = img_width + expand_left + expand_right;

            let mut new_image =
                ImageBuffer::from_pixel(new_width as u32, new_height as u32, Rgb([255, 255, 255]));

            imageops::overlay(
                &mut new_image,
                &self.image,
                expand_left.into(),
                expand_top.into(),
            );

            self.image = new_image;
            self.image_expansion_offset.left += expand_left;
            self.image_expansion_offset.top += expand_top;
        }
    }
}
