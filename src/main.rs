mod pixel_hl;

use std::borrow::Borrow;
use image::{GenericImageView, open, Pixel};

fn main() {
    let image = open("test.png").expect("File not found");
    let pixel: pixel_hl::Pixel = pixel_hl::Pixel::new();
    let mut image_buffer:Vec<[u8; 4]> = Vec::new();

    for i in 0..(image.height() - 1) {
        for j in 0..(image.width() - 1){
            let pixel = image.get_pixel(j,i).borrow().0;
            image_buffer.push(pixel);
            print!("{:?}", pixel)
        }
    }

    println!("{:?}", pixel.get_alpha());
}