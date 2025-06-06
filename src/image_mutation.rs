use image::{DynamicImage, GrayImage, imageops::FilterType};

pub fn resize_image(img: &DynamicImage, visualise: bool) -> DynamicImage {
    let resized = img.resize_exact(9, 8, FilterType::Lanczos3);
    if visualise {
        let rgb = resized.to_rgb8();
        println!("Resized image:");
        for y in 0..rgb.height() {
            for x in 0..rgb.width() {
                let pixel = rgb.get_pixel(x, y);
                let [r, g, b] = pixel.0;

                // Draw pixel with RGB colour
                print!("\x1b[38;2;{r};{g};{b}m█");
            }
            // Reset colours for new line
            println!("\x1b[0m");
        }
    }

    resized
}

pub fn grayscale_image(img: &DynamicImage, visualise: bool) -> GrayImage {
    let gray = img.grayscale().to_luma8();
    if visualise {
        println!("Grayscale Image:");
        for y in 0..gray.height() {
            for x in 0..gray.width() {
                let intensity = gray.get_pixel(x, y)[0];

                // Draw pixel with gray (r=g=b) colour
                print!("\x1b[38;2;{0};{0};{0}m█", intensity);
            }
            // Reset colours for new line
            println!("\x1b[0m");
        }
    }

    gray
}
