use std::borrow::Borrow;
use image::{DynamicImage, GenericImageView};
use crate::pixel_hl::Pixel;

pub struct Image{
    width: u8,
    height: u8,
    pixels: Vec<Pixel>
}

impl Image {
    pub fn new(image: DynamicImage) -> Image{
        let mut image_buffer:Vec<Pixel> = Vec::new();

        for i in 0..(image.height() - 1) {
            for j in 0..(image.width() - 1){
                let color_array = image.get_pixel(j,i).borrow().0;
                let pixel: Pixel = Pixel::new_from_color_array(color_array);
                image_buffer.push(pixel);
            }
        }

        Image {
            width: image.width() as u8,
            height: image.height() as u8,
            pixels: image_buffer
        }
    }
}

// pub fn get_position(x: u8, y: u8) -> Pixel{
//
// }