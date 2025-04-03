use clap::Parser;
use sprite_splitter::SpriteSheet;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(index = 1, help = "Path to the sprite sheet image")]
    file_path: String,
    #[arg(
        short = 'o',
        long = "output",
        default_value = ".",
        help = "Output directory"
    )]
    output_dir: String,
    #[arg(
        short = 'r',
        long = "rows",
        help = "Number of rows in the sprite sheet"
    )]
    rows: u32,
    #[arg(
        short = 'c',
        long = "columns",
        help = "Number of columns in the sprite sheet"
    )]
    columns: u32,
    #[arg(
        short = 'q',
        long = "quiet",
        default_value_t = false,
        help = "Suppress output"
    )]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = fs::metadata(&args.file_path) {
        eprintln!("Error: {}", err);
        return;
    }

    if let Err(err) = fs::create_dir_all(&args.output_dir) {
        eprintln!("Error: {}", err);
        return;
    }

    let file_path = Path::new(&args.file_path);
    let output_dir = Path::new(&args.output_dir);

    let mut ss = SpriteSheet::new(file_path, output_dir, args.rows, args.columns);

    if !args.quiet {
        println!("Image size: ({}, {})", ss.img_width(), ss.img_height());
        println!(
            "Sprite size: ({}, {})",
            ss.sprite_width(),
            ss.sprite_height()
        );
    }

    while ss.next().is_some() {}
}
