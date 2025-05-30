pub struct Framebuffer {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let data = vec![0; (width * height * 4) as usize]; // 4 bytes per pixel (RGBA)
        Framebuffer {
            width,
            height,
            data,
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            self.data[index] = r;
            self.data[index + 1] = g;
            self.data[index + 2] = b;
            self.data[index + 3] = a;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Option<(u8, u8, u8, u8)> {
        if x < self.width && y < self.height {
            let index = ((y * self.width + x) * 4) as usize;
            Some((
                self.data[index],
                self.data[index + 1],
                self.data[index + 2],
                self.data[index + 3],
            ))
        } else {
            None
        }
    }
}
