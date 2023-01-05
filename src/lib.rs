mod errors;
mod hex_math;
mod numgen;
mod utils;

use numgen::PathGenerator;
use pyo3::prelude::*;

pub use numgen::Bounds;

pub fn generate_number_pattern(
    target: i32,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
) -> Option<(String, String)> {
    let path = PathGenerator::new(target, bounds, carryover, trim_larger).run()?;
    Some((path.starting_direction().to_string(), path.pattern()))
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
) -> Option<(String, String)> {
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
    Ok(())
}
