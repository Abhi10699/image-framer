mod merger;
mod overlay;

use clap::Parser;

use crate::overlay::Overlay;

// Simple tool to overlay images on a white backdrop
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Input Image Path
    #[arg(short, long)]
    input: String,

    // Output director Path with output image name eg. ./
    #[arg(short, long)]
    output_path: String,

    // Output director Path with output image name eg. ./
    #[arg(short, long)]
    output_file_name: String,

    // Path of input intro video
    #[arg(short, long)]
    input_video_path: String,
}

fn main() {
    let cli_args = Args::parse();
    let overlayed_image = Overlay::overlay_white_backdrop(&cli_args.input);

    let image_file_path = format!(
        "{}{}.png",
        &cli_args.output_path, &cli_args.output_file_name
    );

    let video_file_path = format!(
        "{}{}.mp4",
        &cli_args.output_path, &cli_args.output_file_name
    );

    overlayed_image.save(&image_file_path).unwrap();

    // Run ffmpeg command
    if merger::merge_mp4_image(image_file_path, &cli_args.input_video_path, video_file_path) {
        println!("Merge Sucessful")
    }
    else {
        println!("Something went wrong!")
    }
}
