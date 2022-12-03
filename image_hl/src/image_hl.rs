use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use image::{DynamicImage, GenericImageView};
use crate::pixel_hl::Pixel;

pub struct Image{
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new_color_array(image: DynamicImage) -> Image{
        let mut image_buffer:Vec<Pixel> = Vec::new();

        for i in 0..(image.height()) {
            for j in 0..(image.width()) {
                let color_array = image.get_pixel(j,i).borrow().0;
                let pixel: Pixel = Pixel::new_from_color_array(color_array);
                image_buffer.push(pixel);
            }
        }

        Image {
            width: image.width(),
            height: image.height(),
            pixels: image_buffer,
        }
    }

    pub fn convert_to_byte_buffer(self: &mut Self) -> Vec<u8>{
        let mut vec_buffer = Vec::with_capacity(self.pixels.len() * 3);
        for i in 0..self.height as usize{
            for j in 0..self.width as usize{
                vec_buffer.push(self.get_pixel(i as u32,j as u32).get_red());
                vec_buffer.push(self.get_pixel(i as u32,j as u32).get_green());
                vec_buffer.push(self.get_pixel(i as u32,j as u32).get_blue());
            }
        }
        vec_buffer
    }

    pub fn save_image_data(self: &mut Self, file_name: &str, byte_buffer: &Vec<u8>) -> std::io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(byte_buffer)?;

        Ok(())
    }

    pub fn get_pixel(self: &mut Self, x: u32, y: u32) -> Pixel{
        let position = ((y * self.width) + x) as usize;
        return self.pixels[position];
    }

    pub fn set_pixel(self: &mut Self, x: u32, y: u32, pixel: Pixel){
        let position = ((y * self.width) + x) as usize;
        self.pixels[position] = pixel;
    }

    //Getters
    pub fn get_width(self: &mut Self) -> u32{ return self.width; }

    pub fn get_height(self: &mut Self) -> u32{ return self.height; }
}
