use rand::prelude::*;
use std::path::Path;
use image::{ImageBuffer,Rgb,RgbImage};
use imageproc::rect::Rect;
use imageproc::point::Point;
use imageproc::drawing::{draw_filled_rect_mut, draw_filled_circle_mut, draw_text_mut, draw_polygon_mut};
use rusttype::{Font, Scale};

// More examples straight from github
// https://github.com/image-rs/imageproc/blob/master/README.md

const IMAGE_WIDTH:u32 = 1024;
const IMAGE_HEIGHT:u32 = 1024;

fn main() {

    let font = Vec::from(include_bytes!("../fonts/FiraSans-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();
    
    let mut rng = rand::thread_rng();
    
    // Some colours to play with, but you can use random colours too
    let colour_red = Rgb::<u8>([183,87,0]);
    let colour_green = Rgb::<u8>([0,183,87]);
    let colour_blue = Rgb::<u8>([0,87,183]);
    let colour_pink = Rgb::<u8>([255,192,203]);
    let colour_random = Rgb::<u8>([rng.gen_range(0..=255),rng.gen_range(0..=255),rng.gen_range(0..=255)]);
    
    // Create a blue image to start with
    let mut img = ImageBuffer::from_pixel(IMAGE_WIDTH, IMAGE_HEIGHT, colour_blue);
    // Or just create a new image without a specific colour
    // let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Set a pixel of a certain colour depending on the result of a function, here using Cosine function
    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {
            if y as f32 > ((x as f32 / 100.0).cos() * 100.0) + IMAGE_HEIGHT as f32 / 2.0 {
                img.put_pixel(x, y, colour_random);
            } else {
                img.put_pixel(x, y, colour_green);
            }
        }
    }

    // For more shapes to draw:
    // https://docs.rs/imageproc/latest/imageproc/drawing/index.html
    let rect = Rect::at(50, 50).of_size(100, 100);
    draw_filled_rect_mut(&mut img, rect, colour_red);

    draw_filled_circle_mut(&mut img, (300, 300), 50, colour_red);

    let scale = Scale {
        x: 40.0,
        y: 40.0,
    };
    draw_text_mut(&mut img, colour_pink, 400, 600, scale, &font, "RUST");

    draw_polygon_mut(
        &mut img,
        &[
            Point::new(380, 80),
            Point::new(480, 80),
            Point::new(580, 220),
            Point::new(680, 220),
        ],
        colour_red,
    );


    // save image
    let mut image_count = 1;
    let mut file_name = "image".to_string() + &image_count.to_string() + &".jpg".to_string();
    loop {
        file_name = "image".to_string() + &image_count.to_string() + &".jpg".to_string();
        if Path::new(&file_name).exists() {
            image_count += 1;
            continue;
        } else {
            break;
        }
    }

    let path = Path::new(&file_name);
    img.save(path).unwrap();

    println!("Art Saved!");
}
