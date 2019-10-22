use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitables::{ HitRecord };

use crate::hitables::Sphere;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p.clone() + rec.normal.clone() + Sphere::random_in_unit_sphere();
        let scattered = Ray::new(&rec.p, &(target - rec.p.clone()));
        
        Some((self.albedo.clone(), scattered))
    }
}
