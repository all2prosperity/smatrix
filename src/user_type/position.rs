use super::determinant::Determinant;
use super::vector::Vector3;
use std::ops::{Sub};

pub struct Pos3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Sub for Pos3 {
    type Output = Vector3;

    fn sub(self, other: Self) -> Vector3 {
        let _determinant = (self.to_determinant() - other.to_determinant()).unwrap();
        Vector3::from_determinant(_determinant)
    }
}


impl Pos3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn to_determinant(&self) -> Determinant {
        let _elements = vec![self.x, self.y, self.z, 1.];
        Determinant::from_vec(4, 1, true, _elements).unwrap()
    }
}
