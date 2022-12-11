use crate::image_hl::Imagehl;
use rand::{Rng,thread_rng};

impl Imagehl{
    pub fn randomize(self: &mut Self){
        let mut rng = thread_rng();
        let rnx= rng.gen_range(0..self.get_width());
        let rny= rng.gen_range(0..self.get_width());
        let rnx_2 = rng.gen_range(0..self.get_width());
        let rny_2 = rng.gen_range(0..self.get_width());

        let pixel_1 = self.get_pixel(rnx,rny);
        let pixel_2 = self.get_pixel(rnx_2,rny_2);

        self.set_pixel(rnx_2,rny_2,pixel_1);
        self.set_pixel(rnx, rny, pixel_2);
    }
}