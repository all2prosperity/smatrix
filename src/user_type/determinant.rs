use std::ops::{Add, Sub};

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
        if self.M() != other.M() {
            None
        }
        else if self.N() != other.N() {
            None
        }
        else {
            let _m = self.M();
            let _n = self.N();
            let _ret = Determinant::new(self.M(), self.N(), false);
            let mut _sIter = self.iter();
            let mut _oIter = other.iter();
            let mut _elements = Vec::new();
            loop {
                if let Some(_item) = _sIter.next() {
                    _elements.push(_item + _oIter.next().unwrap())
                }
                else {
                    break;
                }
            }
            Some(Determinant::from_vec(self.M(), self.N(), false, _elements))
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

    pub fn from_vec(m: u32, n: u32, t: bool, elements: Vec<f32>) -> Self{
        Self {
            m, n, t, elements
        }
    }

    pub fn M(&self) -> u32 {
        if self.t {
            self.n
        }
        else {
            self.m
        }
    }

    pub fn N(&self) -> u32 {
        if self.t {
            self.m
        }
        else {
            self.n
        }
    }

    pub fn iter(&self) -> DeterminantIter {
        DeterminantIter {
            iter: &self,
            x: 0,
            y: 0,
        }
    }

    pub fn index(&self, x: u32, y: u32) -> f32{
        let _idx;
        if self.t {
            _idx = y * self.m + x;
        }
        else {
            _idx = x * self.m + y;
        }

        self.elements[_idx as usize]
    }
}

impl<'a> Iterator for DeterminantIter<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.iter.M() - 1 && self.y == self.iter.N() - 1 {
            None
        }
        else {
            self.y += 1;
            if self.y == self.iter.N() {
                self.y = 0;
                self.x += 1
            }

           Some(self.iter.index(self.x, self.y))
        }
    }
}


