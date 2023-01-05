mod errors;
mod hex_math;
mod numgen;
mod utils;

use numgen::PathGenerator;
use pyo3::prelude::*;

pub use numgen::Bounds;

#[pyclass]
pub struct GeneratedNumber {
    #[pyo3(get)]
    pub direction: String,
    #[pyo3(get)]
    pub pattern: String,
    #[pyo3(get)]
    pub largest_dimension: u32,
    #[pyo3(get)]
    pub num_points: usize,
}

#[pymethods]
impl GeneratedNumber {
    fn __str__(&self) -> String {
        format!("{} {}", self.direction, self.pattern)
    }
}

pub fn generate_number_pattern(
    target: i32,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
) -> Option<GeneratedNumber> {
    let path = PathGenerator::new(target, bounds, carryover, trim_larger).run()?;
    Some(GeneratedNumber {
        direction: path.starting_direction().to_string(),
        pattern: path.pattern(),
        largest_dimension: path.bounds().largest_dimension(),
        num_points: path.num_points(),
    })
}

#[pyfunction]
#[pyo3(name = "generate_number_pattern")]
fn generate_number_pattern_py(
    target: i32,
    q_size: Option<u32>,
    r_size: Option<u32>,
    s_size: Option<u32>,
    carryover: Option<usize>,
    trim_larger: Option<bool>,
) -> Option<GeneratedNumber> {
    generate_number_pattern(
        target,
        Bounds::new(q_size.unwrap_or(8), r_size.unwrap_or(8), s_size.unwrap_or(8)),
        carryover.unwrap_or(25),
        trim_larger.unwrap_or(true),
    )
}

#[pymodule]
fn hexnumgen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_number_pattern_py, m)?)?;
    m.add_class::<GeneratedNumber>()?;
    Ok(())
}
