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

    println!("{}", dhash(&img, true))
}

fn dhash(img: &DynamicImage, visualise: bool) -> u64 {
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

    let gray = resized.grayscale().to_luma8();
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

    let mut hash: u64 = 0;
    if visualise {
        println!("dHashed Image:")
    }
    for y in 0..8 {
        for x in 0..8 {
            let left_pixel_intensity = gray.get_pixel(x, y)[0];
            let right_pixel_intensity = gray.get_pixel(x + 1, y)[0];

            hash <<= 1; // Shift hash bits left
            if left_pixel_intensity > right_pixel_intensity {
                hash |= 1; // Make LSB 1

                if visualise {
                    print!("▉")
                }
            } else if visualise {
                print!(" ")
            }
        }

        if visualise {
            println!()
        }
    }

    hash
}
