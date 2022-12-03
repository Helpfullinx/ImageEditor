extern crate core;

use std::borrow::Borrow;
use image::{GenericImageView, open};
use image_hl::{image_hl::Image};
use image_editor_gui::new_window;

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

    // new_window::new_window();
}