mod compare;
mod hash;
mod visualise;
use clap::Parser;

#[derive(Parser)]
#[command(name = "image-compare")]
struct Cli {
    path1: String,
    path2: String,

    #[arg(short, long, help = "Display visualisations of image calculations")]
    visualise: bool,
}

fn main() {
    let cli = Cli::parse();
    println!(
        "{}%",
        compare::compare_images(&cli.path1, &cli.path2, cli.visualise)
    )
}
