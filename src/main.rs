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

    dhash(&img)
}

fn dhash(img: &DynamicImage) {
    let resized = img.resize_exact(9, 8, FilterType::Lanczos3);
    let rgb = resized.to_rgb8();

    for y in 0..rgb.height() {
        for x in 0..rgb.width() {
            let pixel = rgb.get_pixel(x, y);
            let [r, g, b] = pixel.0;

            print!("\x1b[38;2;{r};{g};{b}m█");
        }
        println!("\x1b[0m");
    }
}
