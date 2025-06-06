use crate::visualise;
use image::{DynamicImage, GenericImageView, imageops::FilterType};

pub fn dhash(img: &DynamicImage, visualise: bool) -> u64 {
    // Mutate image
    let resized = img.resize_exact(9, 8, FilterType::Lanczos3);
    let gray = resized.grayscale();
    if visualise {
        println!("Squash:");
        visualise::print_pixels(&resized);
        println!("Grayscale:");
        visualise::print_pixels(&gray);
    }

    // Calculate hash
    let mut hash: u64 = 0;
    if visualise {
        println!("dHash fingerprint:")
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

pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
