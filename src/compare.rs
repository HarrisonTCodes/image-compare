use crate::hash;

pub fn compare_images(path1: &str, path2: &str) -> f64 {
    let img1 = image::open(path1).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path1, e);
        std::process::exit(1);
    });

    let img2 = image::open(path2).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path2, e);
        std::process::exit(1);
    });

    let dist = hash::hamming_distance(hash::dhash(&img1, true), hash::dhash(&img2, true));

    (1.0 - dist as f64 / 64.0) * 100.0
}
