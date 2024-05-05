use image::{self, imageops, Rgb, RgbImage};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Object {
    w: i32,
    h: i32,
}

fn find_center_position(background: Object, foreground: Object) -> Position {
    // assume they are both aligned at top left (0,0)
    let background_center = Position {
        x: background.w / 2,
        y: background.h / 2,
    };
    let c = Position {
        x: ((background_center.x - foreground.w) + foreground.w / 2),
        y: ((background_center.y - foreground.h) + foreground.h / 2),
    };

    c
}

fn main() {
    let mut image_file = image::open("./test.jpg").unwrap().into_rgb8();
    
    let aspect_ratio: f32 = image_file.height() as f32 / image_file.width() as f32;

    println!(
        "Height: {height}, Width: {width}, aspect_ratio: {ar}",
        height = image_file.height(),
        width = image_file.width(),
        ar = aspect_ratio,
    );

    let mut white_image = RgbImage::from_pixel(1080, 1920, Rgb { 0: [255, 255, 255] });
    
    let new_size = Object {
        w: 970,
        h: (970.0 * aspect_ratio) as i32
    };
    
    
    image_file = imageops::resize(&image_file, new_size.w as u32, new_size.h as u32, imageops::FilterType::CatmullRom);



    let background = Object {
        w: white_image.width() as i32,
        h: white_image.height() as i32,
    };

    let foreground = Object {
        w: image_file.width() as i32,
        h: image_file.height() as i32,
    };

    println!("{:?}", foreground);
    println!("{:?}", background);

    let center_pos = find_center_position(background, foreground);
    println!("{:?}", center_pos);

    imageops::overlay(
        &mut white_image,
        &mut image_file,
        center_pos.x.into(),
        center_pos.y.into(),
    );
    white_image.save("output.png").unwrap();
}
