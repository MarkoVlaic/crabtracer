use super::material::Material;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hitables::HitRecord;
use crate::hitables::Sphere;

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32
}

impl Metal {
    pub fn new(albedo: Vec3, mut fuzz: f32) -> Self {
        if fuzz > 1.0 {
            fuzz = 1.0;
        }
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> { 
        let reflected = Metal::reflect( &Vec3::unit_vector(ray_in.direction()), &rec.normal );
        let scattered = Ray::new(&rec.p, &(reflected + Sphere::random_in_unit_sphere() * self.fuzz));

        if scattered.direction().clone() * rec.normal.clone() > 0.0 {
            return Some((self.albedo.clone(), scattered))
        }

        None
    }
}