use super::determinant;
use determinant::Determinant;


#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub fn to_determinant(&self) -> Determinant {
        let _elements = vec![self.x, self.y, self.z];
        Determinant::from_vec(3, 1, false, _elements).unwrap()
    }

    pub fn to_linear_determinant(&self) -> Determinant {
        let _elements = vec![self.x, self.y, self.z, 0.0];
        Determinant::from_vec(4, 1, false, _elements).unwrap()
    }

    pub fn from_determinant(determinant: Determinant) -> Self {
        Vector3 {
            x: determinant.index(0, 0),
            y: determinant.index(1, 0),
            z: determinant.index(2, 0)
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        let mut _left = self.to_determinant();
        _left.transform_t();
        let _ret = (_left * other.to_determinant()).unwrap();
        _ret.index(0, 0)
    }

    pub fn to_cross_determinant(&self) -> Determinant {
        let _elements = vec![0., -self.z, self.y, self.z, 0., -self.x, -self.y, self.x, 0.];
        Determinant::from_vec(3, 3, false, _elements).unwrap()
    }

    pub fn cross(&self, other: &Self) -> Self {
        let mut _right = other.to_determinant();
        let _result = (self.to_cross_determinant() * _right).unwrap();
        Self::from_determinant(_result)
    }
}
