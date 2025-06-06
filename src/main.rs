mod compare;
mod hash;
mod visualise;

fn main() {
    let path1 = "../../../Downloads/test1.png";
    let path2 = "../../../Downloads/test4.png";

    println!("{}%", compare::compare_images(path1, path2));
}
