use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>
}

impl HitRecord {
    pub fn new(t:f32, p: Vec3, normal: Vec3, material: Box<dyn Material>) -> Self {
        HitRecord { t, p, normal, material }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
