mod pixel_hl;
mod image_hl;

use image::{open};
use crate::image_hl::Image;

fn main() {
    let image = open("test.png").expect("File not found");


    let _image_buffer = Image::new(image);
}