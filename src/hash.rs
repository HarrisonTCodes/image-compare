use crate::image_mutation;
use image::DynamicImage;

pub fn dhash(img: &DynamicImage, visualise: bool) -> u64 {
    // Mutate image
    let resized = image_mutation::resize_image(&img, visualise);
    let gray = image_mutation::grayscale_image(&resized, visualise);

    // Calculate hash
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

pub fn hamming_distance(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
