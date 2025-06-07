use crate::visualise;
use image::{DynamicImage, GenericImageView, imageops::FilterType};
use itertools::izip;

pub fn dhash(img: &DynamicImage, visualise: bool) -> u64 {
    // Mutate image
    let resized = img.resize_exact(9, 8, FilterType::Lanczos3);
    let gray = resized.grayscale();

    // Calculate hash
    let mut hash: u64 = 0;
    for y in 0..8 {
        for x in 0..8 {
            let left_pixel_intensity = gray.get_pixel(x, y)[0];
            let right_pixel_intensity = gray.get_pixel(x + 1, y)[0];

            hash <<= 1; // Shift hash bits left
            if left_pixel_intensity > right_pixel_intensity {
                hash |= 1; // Make LSB 1
            }
        }
    }

    if visualise {
        let squashed_disp = visualise::pixel_display(&resized);
        let gray_disp = visualise::pixel_display(&gray);
        let fingerprint_disp = visualise::u64_fingerprint_display(&hash);

        println!("Squash -> Grayscale -> dHash");
        for (squashed_line, gray_line, fingerprint_line) in izip!(
            squashed_disp.lines(),
            gray_disp.lines(),
            fingerprint_disp.lines()
        ) {
            println!("{} | {} | {}", squashed_line, gray_line, fingerprint_line);
        }
    }

    hash
}

pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
