mod compare;
mod hash;
mod utils;
mod visualise;
use crate::utils::Algorithm;
use clap::Parser;

#[derive(Parser)]
#[command(name = "image-compare")]
struct Cli {
    path1: String,
    path2: String,

    #[arg(short, long, help = "Display visualisations of image calculations")]
    visualise: bool,

    #[arg(
        short,
        long,
        value_enum,
        default_value = "dhash",
        help = "Hashing algorithm"
    )]
    algorithm: Algorithm,
}

fn main() {
    let cli = Cli::parse();
    compare::compare_images(&cli.path1, &cli.path2, cli.visualise, cli.algorithm)
}
