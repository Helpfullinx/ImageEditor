#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    // Create default
    pub fn new() -> Pixel {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255
        }
    }

    // Create new from RGBA
    pub fn new_from_rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a
        }
    }

    // Create new from RGB
    pub fn new_from_rgb(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {
            r,
            g,
            b,
            a: 255
        }
    }

    // Create new from byte buffer
    pub fn new_from_bytes(buffer: u32) -> Pixel {
        Pixel {
            r: (buffer & (0x000000FF)) as u8,
            g: ((buffer & (0x0000FF00)) >> 8) as u8,
            b: ((buffer & (0x00FF0000)) >> 16) as u8,
            a: ((buffer & (0xFF000000)) >> 24) as u8
        }
    }

    pub fn new_from_color_array(array: [u8; 4]) -> Pixel {
        Pixel{
            r: array[0],
            g: array[1],
            b: array[2],
            a: array[3]
        }
    }

    // pub fn get_as_byte(self: &mut Self) -> u32{
    //     let byte:u32;
    //     byte =
    // }

    //Setters
    pub fn set_red(self: &mut Self, r: u8) {
        self.r = r;
    }

    pub fn set_green(self: &mut Self, g: u8) {
        self.g = g;
    }

    pub fn set_blue(self: &mut Self, b: u8) {
        self.b = b;
    }

    pub fn set_alpha(self: &mut Self, a: u8) {
        self.a = a;
    }

    //Getters
    pub fn get_red(self: &Self) -> u8{
        self.r
    }

    pub fn get_green(self: &Self) -> u8{
        self.g
    }

    pub fn get_blue(self: &Self) -> u8{
        self.b
    }

    pub fn get_alpha(self: &Self) -> u8{
        self.a
    }
}