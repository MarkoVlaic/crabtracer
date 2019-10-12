use crate::hitable::{ Hitable, HitRecord };
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }

    pub fn new_boxed(center: Vec3, radius: f32) -> Box<Self> {
        Box::new(Sphere { center, radius })
    }

    fn construct_solution(&self, t: f32, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if t < t_max && t > t_min {
            let normal = (r.point_at_parameter(t) - self.center.clone())/self.radius;
            let record = HitRecord::new(t, r.point_at_parameter(t), normal);
            return Some(record);
        }

        None
    } 
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin().clone() - self.center.clone();
        let a = r.direction().clone() * r.direction().clone();
        let b = oc.clone() * r.direction().clone();
        let c = oc.clone() * oc.clone() - self.radius * self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt())/a;
            let sol = self.construct_solution(temp, r, t_min, t_max);
            if sol.is_some() {
                return sol;
            } else {
                let temp = (-b + (b*b - a*c).sqrt())/a;
                let sol = self.construct_solution(temp, r, t_min, t_max);
                return sol;
            }
        }

        None
    }
}

