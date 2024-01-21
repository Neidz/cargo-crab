use image::Rgb;

pub struct ColorUtils;

impl ColorUtils {
    pub fn equal_with_tolerance(color1: &Rgb<u8>, color2: &Rgb<u8>, tolerance: u8) -> bool {
        let Rgb([r1, g1, b1]) = color1;
        let Rgb([r2, g2, b2]) = color2;

        let diff_r = safe_abs(r1, r2);
        let diff_g = safe_abs(g1, g2);
        let diff_b = safe_abs(b1, b2);

        diff_r <= tolerance && diff_g <= tolerance && diff_b <= tolerance
    }
}

fn safe_abs(num1: &u8, num2: &u8) -> u8 {
    if num1 < num2 {
        return num2 - num1;
    }
    num1 - num2
}
