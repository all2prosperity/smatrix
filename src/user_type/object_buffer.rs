use super::position::Pos3;

pub struct Triangle {
    pub pos1: Pos3,
    pub pos2: Pos3,
    pub pos3: Pos3,
}

impl Triangle {
    pub fn new(pos1: Pos3, pos2: Pos3, pos3: Pos3) -> Self {
        Self {
            pos1, pos2, pos3
        }
    }
}

pub struct ObjectBuffer {
    pub
    buffer: Vec<Triangle>   
}

impl ObjectBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }

    pub fn add_object(&mut self, obj: Triangle) -> &mut Self {
        self.buffer.push(obj);
        self
    }
}
