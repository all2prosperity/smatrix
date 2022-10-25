use std::ops::{Add};

pub struct Determinant {
// 0 1 2  m = 2, n = 3
// 0 1 2
//
//
    m: u32,
    n: u32,
    t: bool,
    elements: Vec<f32>,
}

pub struct DeterminantIter<'a> {
    iter: &'a Determinant,
    x: u32,
    y: u32,
}

impl<'a> DeterminantIter<'a> {
    fn new(iter: &'a Determinant, x: u32, y: u32) -> Self{
        DeterminantIter {
            iter, x, y
        }
    }
}

impl Add for Determinant {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Option<Self> {
        if self.m() != other.m() {
            None
        }
        else if self.n() != other.n() {
            None
        }
        else {
            let mut _s_iter = self.iter();
            let mut _o_iter = other.iter();
            let mut _elements = Vec::new();
            loop {
                if let Some(_item) = _s_iter.next() {
                    _elements.push(_item + _o_iter.next().unwrap())
                }
                else {
                    break;
                }
            }
            Determinant::from_vec(self.m(), self.n(), false, _elements)
        }
    }
}

impl Determinant {
    pub fn new(m: u32, n: u32, t: bool) -> Self{
        Self {
            m, n, t, 
            elements: Vec::with_capacity((m * n) as usize), 
        }
    }

    pub fn from_vec(m: u32, n: u32, t: bool, elements: Vec<f32>) -> Option<Self>{
        if m * n != elements.len() as u32 {
            None
        }
        else {
            Some(Self {
                m, n, t, elements
            })
        }
    }

    pub fn debug(&self) {
        let mut _print = String::from("");
        for i in 0..(self.m()) {
            for j in 0..(self.n()) {
                _print.push(' ');
                _print += &self.index(i, j).to_string();
            }

            _print.push('\n');
        }

        println!("{}", _print);
    }

    pub fn m(&self) -> u32 {
        if self.t {
            self.n
        }
        else {
            self.m
        }
    }

    pub fn n(&self) -> u32 {
        if self.t {
            self.m
        }
        else {
            self.n
        }
    }

    pub fn t(&self) -> Self {
        Determinant {
            m: self.m,
            n: self.n,
            elements: self.elements.clone(),
            t: !self.t,
        }
    }

    pub fn iter(&self) -> DeterminantIter {
        DeterminantIter::new(self, 0, 0)
    }

    pub fn index(&self, x: u32, y: u32) -> f32{
        let _idx;
        if self.t {
            _idx = y * self.n + x;
        }
        else {
            _idx = x * self.n + y;
        }

        self.elements[_idx as usize]
    }
}

impl<'a> Iterator for DeterminantIter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.iter.m() {
            None
        }
        else {
            let _ret = self.iter.index(self.x, self.y);

            self.y += 1;
            if self.y == self.iter.n() {
                self.y = 0;
                self.x += 1
            }

            Some(_ret)
        }
    }
}


