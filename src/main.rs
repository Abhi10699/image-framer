use clap::Parser;
use image::{self, imageops, ImageBuffer, Rgb, RgbImage};

// Simple tool to overlay images on a white backdrop
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input Image Path
    #[arg(short, long)]
    input: String,

    /// Output Image Path with output image name eg. ./output.png
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

fn calculate_overlay_size(
    overlay_image: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    target_width: i32,
) -> ImageDimension {
    let aspect_ratio: f32 = overlay_image.height() as f32 / overlay_image.width() as f32;
    ImageDimension {
        w: target_width,
        h: (target_width as f32 * aspect_ratio) as i32,
    }
}

fn main() {
    let cli_args = Args::parse();

    const TARGET_WIDTH: i32 = 970;
    let mut foreground_image = image::open(cli_args.input).unwrap().into_rgb8();
    let mut white_image = RgbImage::from_pixel(1080, 1920, Rgb { 0: [255, 255, 255] });

    let new_size = calculate_overlay_size(&foreground_image, TARGET_WIDTH);

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
    white_image.save(cli_args.output).unwrap();
}
