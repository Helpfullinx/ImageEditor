extern crate core;

mod pixel_hl;
mod image_hl;

use std::borrow::Borrow;
use image::{GenericImageView, open};
use crate::image_hl::Image;
use crate::pixel_hl::Pixel;


fn main() {
    let dynamic_image = open("test.png").expect("File not found");

    let mut image = Image::new_color_array(dynamic_image);

    let mut i = 0;
    while i < 1440000 {
        image.randomize();
        i += 1;
    }

    let byte_buffer = image.convert_to_byte_buffer();

    image.save_image_data("image.data",&byte_buffer);
}