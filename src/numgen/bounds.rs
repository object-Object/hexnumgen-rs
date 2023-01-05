#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bounds {
    q: u32,
    r: u32,
    s: u32,
}

impl Bounds {
    pub fn new(q: u32, r: u32, s: u32) -> Self {
        Self { q, r, s }
    }

    pub fn quasi_area(&self) -> u32 {
        self.q * self.r
    }

    pub fn is_better_than(&self, other: Self) -> bool {
        self.quasi_area() < other.quasi_area()
    }

    pub fn fits_in(&self, other: Self) -> bool {
        self.q <= other.q && self.r <= other.r && self.s <= other.s
    }

    pub fn largest_dimension(&self) -> u32 {
        *[self.q, self.r, self.r].iter().max().unwrap()
    }
}

impl From<u32> for Bounds {
    fn from(size: u32) -> Self {
        Self::new(size, size, size)
    }
}
