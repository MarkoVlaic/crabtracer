use crate::hitable::{ Hitable, HitRecord };
use crate::ray::Ray;

type hit_vec = Vec<Box<dyn Hitable>>;

pub struct HitableList {
    list: hit_vec
}

impl HitableList {
    pub fn new(list: hit_vec) -> Self {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest = t_max;

        for object in self.list.iter() {
            if let Some(new_hit) = object.hit(ray, t_min, closest) {
                closest = new_hit.t;
                temp_rec = Some(new_hit);
            }
        }

        temp_rec
    }
}