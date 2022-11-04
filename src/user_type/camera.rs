use super::position::Pos3;
use super::matrix::Matrix;

pub struct Camera {
    fov_y: f32,
    ratio: f32,
    n: f32,
}

impl Camera {
    pub fn new(fov_y: f32, ratio: f32, n: f32) -> Self{
        Self {
            fov_y,
            ratio,
            n
        }
    }

    pub fn transform(&self, pos: &Pos3) ->  Option<Pos3>{
        if pos.z > self.n {
            return None;
        }

        let persp = Matrix::from_vec(4, 4, false, 
                         vec![
                            self.n, 0., 0., 0.,
                            0., self.n, 0., 0.,
                            0., 0., self.n + pos.z, - self.n * pos.z,
                            0., 0., 1., 0.,
                         ]).unwrap();

        let fov_x = self.fov_y * self.ratio;

        let (n, f, l, r, b, t) = (self.n, pos.z, -fov_x / 2., fov_x / 2., -self.fov_y / 2., self.fov_y / 2.); 
        let ort1 = Matrix::from_vec(4, 4, false,
                        vec![
                            2. / (r - l), 0., 0., 0.,
                            0., 2. / (t - b), 0., 0.,
                            0., 0., 2. / (n - f), 0.,
                            0., 0., 0., 1.,
                        ]).unwrap();

        let ort2 = Matrix::from_vec(4, 4, false, 
                        vec![
                            1., 0., 0., -(r + l) / 2.,
                            0., 1., 0., -(t + b) / 2.,
                            0., 0., 1., -(n + f) / 2.,
                            0., 0., 0., 1.,
                        ]).unwrap();

        if let Some(_mat) = ((persp * ort1)? * ort2)? * pos.to_matrix() {
            Some(Pos3::from_matrix(_mat))
        }
        else {
            None
        }
    }
}
