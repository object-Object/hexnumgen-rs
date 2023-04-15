use clap::Args;
use pyo3::prelude::*;

#[pyclass(get_all, set_all)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Args)]
pub struct Bounds {
    #[arg(short, long = "q_size", default_value_t = 8)]
    q: u32,
    #[arg(short, long = "r_size", default_value_t = 8)]
    r: u32,
    #[arg(short, long = "s_size", default_value_t = 8)]
    s: u32,
}

#[pymethods]
impl Bounds {
    #[new]
    pub fn new(q: u32, r: u32, s: u32) -> Self {
        Self { q, r, s }
    }

    #[getter]
    pub fn largest_dimension(&self) -> u32 {
        self.q.max(self.r).max(self.s)
    }

    #[getter]
    pub fn quasi_area(&self) -> u32 {
        self.q * self.r * self.s
    }
}

impl Bounds {
    pub fn is_better_than(&self, other: Self) -> bool {
        self.quasi_area() < other.quasi_area()
    }

    pub fn fits_in(&self, other: Self) -> bool {
        self.q <= other.q && self.r <= other.r && self.s <= other.s
    }
}

impl From<u32> for Bounds {
    fn from(size: u32) -> Self {
        Self::new(size, size, size)
    }
}
