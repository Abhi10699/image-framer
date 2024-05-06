mod overlay;

use clap::Parser;
use std::process::Command;

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

fn merge_mp4_image(image_file_path: String, cli_args: Args, video_file_path: String) {
    match Command::new("ffmpeg")
        .args([
            "-loop",
            "1",
            "-framerate",
            "30",
            "-t",
            "5",
            "-i",
            &image_file_path,
            "-i",
            &cli_args.input_video_path,
            "-filter_complex",
            "[1][0]concat=n=2:v=1:a=0",
            &video_file_path,
        ])
        .output()
    {
        Ok(_) => {
            println!("Merge successful")
        }
        Err(message) => {
            println!("{}", message.to_string())
        }
    };
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

    merge_mp4_image(image_file_path, cli_args, video_file_path);
}
