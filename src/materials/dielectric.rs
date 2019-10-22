use super::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::hitables::HitRecord;

use rand::Rng;

#[derive(Clone)]
pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Dielectric { ref_idx }
    }

    fn schlick(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = Dielectric::reflect(&ray_in.direction(), &rec.normal);
        let outward_normal;
        let mut ni_over_nt;

        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let reflect_prob;
        let cosine;

        if ray_in.direction().clone() * rec.normal.clone() > 0.0 {
            outward_normal = -rec.normal.clone();
            ni_over_nt = self.ref_idx;

            cosine = self.ref_idx * (ray_in.direction().clone() * rec.normal.clone()) / ray_in.direction().length();
        } else {
            outward_normal = rec.normal.clone();
            ni_over_nt = 1.0 / self.ref_idx;
            
            cosine = -(ray_in.direction().clone() * rec.normal.clone()) / ray_in.direction().length();
        }

        if let Some(refracted) = Dielectric::refract(&ray_in.direction(), &outward_normal, ni_over_nt) {
            /*let scattered = Ray::new(&rec.p, &refracted);
            return Some((attenuation, scattered));*/
            reflect_prob = Dielectric::schlick(cosine, self.ref_idx);

            let mut rng = rand::thread_rng();
            let rand: f32 = rng.gen();

            if rand > reflect_prob {
                let scattered = Ray::new(&rec.p, &refracted);
                return Some((attenuation, scattered));
            }

        }

        let scattered = Ray::new(&rec.p, &reflected);
        Some((attenuation, scattered))   
    
    }
}