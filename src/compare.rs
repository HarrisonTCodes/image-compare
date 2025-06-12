use crate::{hash, utils};
use colored::*;

pub fn compare_images(path1: &str, path2: &str, visualise: bool, algorithm: utils::Algorithm) {
    // Compute hashes
    let hash1 = path_to_hash(path1, visualise, algorithm);
    let hash2 = path_to_hash(path2, visualise, algorithm);

    // Output hashes
    println!("{} {} fingerprint - {}", path1.bold(), algorithm, hash1);
    println!("{} {} fingerprint - {}", path2.bold(), algorithm, hash2);

    // Compute and output hamming distance
    let dist = utils::hamming_distance(hash1, hash2);
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

fn path_to_hash(path: &str, visualise: bool, algorithm: utils::Algorithm) -> u64 {
    // Get image from path
    let img = image::open(path).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path, e);
        std::process::exit(1);
    });

    // Compute and output hash
    let hash = match algorithm {
        utils::Algorithm::AHash => hash::ahash(&img, visualise),

        utils::Algorithm::DHash => hash::dhash(&img, visualise),
    };

    hash
}
