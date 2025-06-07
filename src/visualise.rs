use image::{DynamicImage, GenericImageView};

pub fn pixel_display(img: &DynamicImage) -> String {
    let mut disp = String::new();

    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _a] = pixel.0;

            // Draw pixel with RGB colour
            disp.push_str(&format!("\x1b[38;2;{};{};{}m█", r, g, b));
        }
        // Reset colours for new line
        disp.push_str("\x1b[0m\n");
    }

    disp
}
