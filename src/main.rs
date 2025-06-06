mod hash;
mod image_mutation;

fn main() {
    let path1 = "../../../Downloads/test1.png";
    let path2 = "../../../Downloads/test4.png";

    let img1 = image::open(path1).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path1, e);
        std::process::exit(1);
    });

    let img2 = image::open(path2).unwrap_or_else(|e| {
        eprintln!("Error opening image {}: {}", path1, e);
        std::process::exit(1);
    });

    let dist = hash::hamming_distance(hash::dhash(&img1, true), hash::dhash(&img2, true));

    println!("{}%", (1.0 - dist as f64 / 64.0) * 100.0);
}
