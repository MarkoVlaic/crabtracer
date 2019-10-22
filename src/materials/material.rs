use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitables::HitRecord;

use objekt;

pub trait Material: objekt::Clone {
    // Returns an option of (attenuation, scattered) 
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;

    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 where Self: Sized {
        v.clone() - 2.0 * (v.clone() * n.clone()) * n.clone()
    }

    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> where Self: Sized {
        let uv = Vec3::unit_vector(v);
        let dt = uv.clone() * n.clone();
        let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
        
        if discriminant > 0.0 {
            let refracted = ni_over_nt * (uv.clone() - n.clone()*dt) - n.clone() * discriminant.sqrt();
            return Some(refracted)
        }

        None
    }
}
