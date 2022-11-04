use super::position::Pos3;
use super::matrix::Matrix;
use super::object_buffer::{ObjectBuffer, Triangle};
use image::{Rgba, ImageBuffer};
use super::output_buffer::{OutputBuffer};

pub struct Camera {
    fov_y: f32,
    ratio: f32,
    n: f32,
    z: f32,
}

impl Camera {
    pub fn new(fov_y: f32, ratio: f32, n: f32, z: f32) -> Self{
        Self {
            fov_y,
            ratio,
            n,
            z,
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
                            0., 0., self.n + self.z, - self.n * self.z,
                            0., 0., 1., 0.,
                         ]).unwrap();

        let fov_x = self.fov_y * self.ratio;

        let (n, f, l, r, b, t) = (self.n, self.z, -fov_x / 2., fov_x / 2., -self.fov_y / 2., self.fov_y / 2.); 
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

        if let Some(_mat) = ((ort1 * ort2)? * persp)? * pos.to_matrix() {
            Some(Pos3::from_matrix(_mat))
        }
        else {
            println!("im 2");
            None
        }
    }

    pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer) -> OutputBuffer {
        let _out = OutputBuffer::new(width, height);

        for _tri in object_buffer.buffer.iter() {
            let _transform = self.transform(&_tri.pos1); 
            println!("{:?}", _transform);
        }

        _out   
    }
}
