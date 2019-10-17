use crate::hitable::{ Hitable, HitRecord };
use crate::ray::Ray;

type HitVec = Vec<Box<dyn Hitable>>;

pub struct HitableList {
    list: HitVec
}

impl HitableList {
    pub fn new(list: HitVec) -> Self {
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