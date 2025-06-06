mod hash;
mod image_mutation;

fn main() {
    let path1 = "../../../Downloads/test1.png";

    let img1 = match image::open(path1) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let dhash1 = hash::dhash(&img1, true);

    let path2 = "../../../Downloads/test4.png";

    let img2 = match image::open(path2) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let dhash2 = hash::dhash(&img2, true);

    let dist = hash::hamming_distance(dhash1, dhash2);

    println!("{}%", (1.0 - dist as f64 / 64.0) * 100.0);
}
