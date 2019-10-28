use crate::vec3::Vec3;
use crate::camera::Camera;
use crate::hitables::{ Hitable, HitableList };
use crate::ray::Ray;

use image::{ ImageBuffer, RgbImage };
use rand::Rng;

pub struct Renderer {
    img: RgbImage,
    width: u32,
    height: u32,
    sample_rate: u32
}

impl Renderer {
    pub fn new(width: u32, height: u32, sample_rate: u32) -> Self {
        Renderer { img: ImageBuffer::new(width, height), width, height, sample_rate }
    }

    pub fn render_image<F>(&mut self, cam: &Camera, world: &HitableList, pixel_processed: F) 
        where F: Fn(u32, u32) -> () {
        
        let mut rng = rand::thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut col = Vec3::new(0.0, 0.0, 0.0);
                //spinner.update(format!("Processing {}/{}", y*width + x + 1, width*height));
                for _ in 0..self.sample_rate {
                    let f1: f32 = rng.gen();
                    let f2: f32 = rng.gen();

                    let u = (x as f32 + f1)/ self.width as f32;
                    let v = (y as f32 + f2)/ self.height as f32;
                    let r = cam.get_ray(u, v);

                    col = col + self.color(&r, world, 0).sqrt() * 255.0;
                }

                col = col.clone()/(self.sample_rate as f32);

                self.draw_pixel(x, y, &col);
                pixel_processed(x, y);
            }
        }
    }


    fn color(&self, r: &Ray, world: &impl Hitable, depth: u8) -> Vec3 {

        if let Some(record) = world.hit(&r, 0.001, std::f32::MAX) {

            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }

            if let Some( (attenuation, scattered) ) = record.material.scatter(r, &record) {
                return self.color(&scattered, world, depth+1).mul(&attenuation);
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        return Vec3::new(1.0, 1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }

    fn draw_pixel(&mut self, x:u32, y:u32, color: &Vec3) {
        self.img.put_pixel(x, self.height - y - 1, image::Rgb([color.x as u8, color.y as u8, color.z as u8]));
    }

    pub fn output_image(&self, filename: &'static str) {
        self.img.save(filename).unwrap()
    }
}