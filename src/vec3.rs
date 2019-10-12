use std::ops;

#[derive(PartialEq, Debug, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v.clone() / v.clone().length()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f32;

    fn mul(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 { x: self.x/other, y: self.y/other, z: self.z/other }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_vecs() -> (Vec3, Vec3) {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);

        (v1, v2)
    }

    #[test]
    fn add() {
        let (v1, v2) = init_vecs();
        let v3 = v1 + v2;

        assert_eq!(v3, Vec3::new(5.0, 7.0, 9.0));

        assert_eq!(v1.x, 1.0);
    }

    #[test]
    fn sub() {
        let (v1, v2) = init_vecs();
        let v3 = v2 - v1;
        
        assert_eq!(v3, Vec3::new(3.0, 3.0, 3.0));
    }

    fn scalar() {
        let (v1, _) = init_vecs();
        let v3 = v1 * 2.0;

        assert_eq!(v3, Vec3::new(2.0, 4.0, 6.0));
    }

    fn dot() {
        let (v1, v2) = init_vecs();
        let v3 = v1 * v2;

        assert_eq!(v3, 32.0);
    }
}