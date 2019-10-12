use crate::vec3::Vec3;

use image::{ ImageBuffer, RgbImage };

pub struct Renderer {
    img: RgbImage,
    height: u32
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Renderer { img: ImageBuffer::new(width, height), height }
    }

    pub fn draw(&mut self, x:u32, y:u32, color: &Vec3) {
        self.img.put_pixel(x, self.height - y - 1, image::Rgb([color.x as u8, color.y as u8, color.z as u8]));
    }

    pub fn render(&self, filename: &'static str) {
        self.img.save(filename).unwrap()
    }
}