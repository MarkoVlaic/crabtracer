use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    A: Vec3,
    B: Vec3
}

impl Ray {
    pub fn new(origin: &Vec3, direction: &Vec3) -> Self {
        Ray { A: origin.clone(), B: direction.clone() }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.A
    }

    pub fn direction(&self) -> &Vec3 {
        &self.B
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.A.clone() + self.B.clone() * t
    }
}