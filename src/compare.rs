use crate::hash;
use colored::*;

pub fn compare_images(path1: &str, path2: &str, visualise: bool) {
    let img1 = image::open(path1).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path1, e);
        std::process::exit(1);
    });
    let img2 = image::open(path2).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path2, e);
        std::process::exit(1);
    });

    let hash1 = hash::dhash(&img1, visualise);
    let hash2 = hash::dhash(&img2, visualise);

    let dist = hash::hamming_distance(hash1, hash2);
    let percentage = (1.0 - dist as f64 / 64.0) * 100.0;

    let message = match percentage {
        p if p == 100.0 => format!("{}% - Images are indistinguishable", p)
            .green()
            .bold(),
        p if p >= 90.0 => format!("{}% - Images are very likely the same", p)
            .green()
            .bold(),
        p if p >= 80.0 => format!("{}% - Images are likely the same", p).green(),
        p if p >= 65.0 => format!("{}% - Images are possibly the same", p).yellow(),
        p => format!("{}% - Images are likely not the same", p)
            .red()
            .bold(),
    };

    println!("{}", message)
}
