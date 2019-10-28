use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, vup: &Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from.clone();
        let w = Vec3::unit_vector(&(look_from.clone() - look_at.clone()));
        let u = Vec3::unit_vector(&Vec3::cross(vup, &w));
        let v = Vec3::cross(&w, &u);

        let lower_left_corner = origin.clone() - half_width*u.clone() - half_height*v.clone() - w.clone();
        let horizontal = 2.0*half_width*u.clone();
        let vertical = 2.0*half_height*v.clone();

        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner.clone() + self.horizontal.clone()*u + self.vertical.clone()*v - self.origin.clone();
        Ray::new(&self.origin, &direction)
    }
}