use clap::Parser;
use image::{self, imageops, ImageBuffer, Rgb, RgbImage};

// Simple tool to overlay images on a white backdrop
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input Image Path
    #[arg(short, long)]
    input: String,

    /// Output Image Path
    #[arg(short, long)]
    output: String,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct ImageDimension {
    w: i32,
    h: i32,
}

impl ImageDimension {
    fn from_image_buffer(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageDimension {
        ImageDimension {
            w: image.width() as i32,
            h: image.height() as i32,
        }
    }
}

fn find_center_position(background: ImageDimension, foreground: ImageDimension) -> Position {
    // assume they are both aligned at top left (0,0)
    let background_center = Position {
        x: background.w / 2,
        y: background.h / 2,
    };
    Position {
        x: ((background_center.x - foreground.w) + foreground.w / 2),
        y: ((background_center.y - foreground.h) + foreground.h / 2),
    }
}

fn main() {
    let cli_args = Args::parse();

    let mut foreground_image = image::open(cli_args.input).unwrap().into_rgb8();

    let aspect_ratio: f32 = foreground_image.height() as f32 / foreground_image.width() as f32;

    println!(
        "Height: {height}, Width: {width}, aspect_ratio: {ar}",
        height = foreground_image.height(),
        width = foreground_image.width(),
        ar = aspect_ratio,
    );

    let mut white_image = RgbImage::from_pixel(1080, 1920, Rgb { 0: [255, 255, 255] });

    let new_size = ImageDimension {
        w: 970,
        h: (970.0 * aspect_ratio) as i32,
    };

    foreground_image = imageops::resize(
        &foreground_image,
        new_size.w as u32,
        new_size.h as u32,
        imageops::FilterType::CatmullRom,
    );

    let background = ImageDimension::from_image_buffer(&white_image);

    let foreground = ImageDimension::from_image_buffer(&foreground_image);

    let center_pos = find_center_position(background, foreground);

    imageops::overlay(
        &mut white_image,
        &mut foreground_image,
        center_pos.x.into(),
        center_pos.y.into(),
    );
    white_image.save(cli_args.output + "output.png").unwrap();
}
