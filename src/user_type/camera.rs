use super::position::Pos3;
use super::matrix::Matrix;
use super::object_buffer::{ObjectBuffer};
use super::triangle::Triangle;
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

    pub fn to_transform_matrix(&self) -> Matrix {
        let fov_x = self.fov_y * self.ratio;
        let (n, f, l, r, b, t) = (self.n, self.z, -fov_x / 2., fov_x / 2., -self.fov_y / 2., self.fov_y / 2.); 

        let persp = Matrix::from_vec(4, 4, false, 
                         vec![
                            self.n, 0., 0., 0.,
                            0., self.n, 0., 0.,
                            0., 0., self.n + self.z, - self.n * self.z,
                            0., 0., 1., 0.,
                         ]).unwrap();

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

        ((ort1 * ort2).unwrap() * persp).unwrap() 
    }

    pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer) -> OutputBuffer {
        let mut _out = OutputBuffer::new(width, height);
        let transform_matrix = self.to_transform_matrix();

        for _tri in object_buffer.iter() {
            let trans_poses = _tri.poses.iter().map(|x| (&transform_matrix * &(x.to_matrix())).unwrap());
            let trans_poses = trans_poses.map(|x| Pos3::from_matrix(&x));
            for pos in trans_poses.clone() {
                if pos.x < -1. || pos.x > 1. || pos.y > 1. || pos.y < -1.{
                    println!("will return: {:?}", pos);
                    return _out;
                }
            }

            let surface_tri_zero = Triangle::from_vec(
                trans_poses.clone().map(|x| _out.pos_to_pixel_pos(&x)).collect()
                );

            let surface_tri_tilt = Triangle::from_vec(
                trans_poses.map(|x| _out.pos_to_pixel_pos_with_z(&x)).collect()
                );

            // println!("surface tri {:?}", surface_tri_tilt);

            let (sx, ex, sy, ey) = surface_tri_zero.get_edge();
            let depth_matrix = surface_tri_tilt.get_depth_matrix();
            // println!("edge :{:?}", (sx, ex, sy, ey));
            // let pos = Pos3::new(330., 420., 0.);
            // let ret = surface_tri_zero.in_triangle(&pos);
            // println!("ret is {:?}", ret);
            for i in sx..ex {
                for j in sy..ey {
                    let pos = Pos3::new(i as f32 + 0.5, j as f32 + 0.5, 0.);
                    if surface_tri_zero.in_triangle(&pos) {
                        let depth = (&depth_matrix * &pos.to_matrix()).unwrap().result();
                        let cur_depth = _out.get_depth(i as usize, j as usize);
                        if depth > cur_depth {
                            _out.set_depth(i as usize, j as usize, depth);
                            let color = (255 as f32 * (depth + 1.) / 2.).floor() as u8;
                            // println!("depth:{:?}, {:?}", depth, color);
                            _out.put_pixel(i, j, &[color, color, color, color]);
                        }
                    }
                }
            }
            // println!("edge2 :{:?}", (sx, ex, sy, ey));
        }

        _out   
    }
}
