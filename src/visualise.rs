use image::{DynamicImage, GenericImageView};

pub fn print_pixels(img: &DynamicImage) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            let [r, g, b, _a] = pixel.0;

            // Draw pixel with RGB colour
            print!("\x1b[38;2;{r};{g};{b}m█");
        }
        // Reset colours for new line
        println!("\x1b[0m");
    }
}
