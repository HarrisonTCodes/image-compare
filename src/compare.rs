use crate::hash;
use colored::*;

pub fn compare_images(path1: &str, path2: &str, visualise: bool) {
    // Get images
    let img1 = image::open(path1).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path1, e);
        std::process::exit(1);
    });
    let img2 = image::open(path2).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path2, e);
        std::process::exit(1);
    });

    // Compute and output hashes
    let hash1 = hash::dhash(&img1, visualise);
    let hash2 = hash::dhash(&img2, visualise);
    println!("{} dHash fingerprint - {}", path1.bold(), hash1);
    println!("{} dHash fingerprint - {}", path2.bold(), hash2);

    // Compute and output hamming distance
    let dist = hash::hamming_distance(hash1, hash2);
    let mut message = format!("Hamming distance {}/64 - ", dist);

    match dist {
        0 => {
            message.push_str("images are indistinguishable");
            message = message.green().bold().to_string();
        }

        1..=5 => {
            message.push_str("images are very likely the same");
            message = message.green().bold().to_string();
        }

        6..=10 => {
            message.push_str("images are likely the same");
            message = message.green().to_string();
        }

        11..=15 => {
            message.push_str("images possibly the same");
            message = message.yellow().to_string();
        }

        _ => {
            message.push_str("images are likely not the same");
            message = message.red().bold().to_string();
        }
    }

    println!("{}", message)
}
