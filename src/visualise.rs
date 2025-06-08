use colored::*;
use image::{DynamicImage, GenericImageView};

pub fn pixel_display(img: &DynamicImage) -> String {
    let mut disp = String::new();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _a] = pixel.0;

            // Draw pixel with RGB colour
            disp.push_str(&format!("\x1b[38;2;{};{};{}m██", r, g, b));
        }
        // Reset colours for new line
        disp.push_str("\x1b[0m\n");
    }

    disp
}

pub fn u64_fingerprint_display(fingerprint: &u64) -> String {
    let mut disp = String::new();

    for y in 0..8 {
        for x in 0..8 {
            let bit = ((fingerprint >> (x + y * 8)) & 1) as u8;
            let pixel = if bit == 1 {
                "1 ".bright_black().on_white().to_string()
            } else {
                "0 ".bright_black().to_string()
            };
            disp.push_str(&pixel);
        }
        disp.push('\n');
    }

    disp
}
