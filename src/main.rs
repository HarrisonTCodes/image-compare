use image::GenericImageView;

fn main() {
    let path = "../../../Downloads/carrots1.webp";

    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let dimensions = img.dimensions();
    let pixel = img.get_pixel(dimensions.0 / 2, dimensions.1 / 2);

    println!("{:?}", pixel);
}
