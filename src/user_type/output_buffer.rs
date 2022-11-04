use image;
pub type Display = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub struct OutputBuffer {
    width: u32,
    height: u32,
    pub display: Display,
    depth: Vec<f32>,
}


impl OutputBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut _depth: Vec<f32> = Vec::with_capacity((width * height) as usize);
        _depth.resize((width * height) as usize, -2.);
        Self {
            width, height,
            display: image::ImageBuffer::new(width, height),
            depth: _depth,
        }
    }

    pub fn get_depth(x: usize, y: usize) {
        
    }
}
