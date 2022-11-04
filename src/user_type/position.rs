use super::matrix::Matrix;
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
        let _matrix = (self.to_matrix() - other.to_matrix()).unwrap();
        Vector3::from_matrix(_matrix)
    }
}


impl Pos3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn from_matrix(matrix: Matrix) -> Self {
        Self {
            x: matrix.index(0, 0),
            y: matrix.index(1, 0),
            z: matrix.index(2, 0),
        }
    }

    pub fn to_matrix(&self) -> Matrix {
        let _elements = vec![self.x, self.y, self.z, 1.];
        Matrix::from_vec(4, 1, true, _elements).unwrap()
    }
}
