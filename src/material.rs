use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitable::HitRecord;

use objekt;

pub trait Material: objekt::Clone {
    // Returns an option of (attenuation, scattered) 
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}
