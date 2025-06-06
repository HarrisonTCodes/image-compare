use image::{DynamicImage, imageops::FilterType};

fn main() {
    let path = "../../../Downloads/carrots1.webp";

    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    dhash(&img, true)
}

fn dhash(img: &DynamicImage, visualise: bool) {
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
}
