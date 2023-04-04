mod errors;
mod hex_math;
mod numgen;
mod traits;
mod utils;

use num_rational::Ratio;
use numgen::{AStarPathGenerator, BeamPathGenerator};
use pyo3::prelude::*;

pub use hex_math::Direction;
pub use numgen::Bounds;

#[derive(FromPyObject)]
pub enum PyRatio {
    #[pyo3(annotation = "int")]
    Int(i64),
    #[pyo3(annotation = "tuple[int, int]")]
    Tuple(i64, i64),
}

impl From<PyRatio> for Ratio<i64> {
    fn from(value: PyRatio) -> Self {
        match value {
            PyRatio::Int(n) => n.into(),
            PyRatio::Tuple(numer, denom) => Ratio::new(numer, denom),
        }
    }
}

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

pub fn generate_number_pattern_beam(
    target: Ratio<i64>,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
    allow_fractions: bool,
) -> Option<GeneratedNumber> {
    let path = BeamPathGenerator::new(target, bounds, carryover, trim_larger, allow_fractions).run()?;
    Some(GeneratedNumber {
        direction: path.starting_direction().to_string(),
        pattern: path.pattern(),
        largest_dimension: path.bounds().largest_dimension(),
        num_points: path.num_points(),
    })
}

pub fn generate_number_pattern_astar(
    target: Ratio<i64>,
    trim_larger: bool,
    allow_fractions: bool,
) -> Option<GeneratedNumber> {
    let path = AStarPathGenerator::new(target, trim_larger, allow_fractions).run()?;
    Some(GeneratedNumber {
        direction: path.starting_direction().to_string(),
        pattern: path.pattern(),
        largest_dimension: path.bounds().largest_dimension(),
        num_points: path.num_points(),
    })
}

#[pyfunction]
#[pyo3(name = "generate_number_pattern_beam")]
#[allow(clippy::too_many_arguments)]
fn generate_number_pattern_beam_py(
    target: PyRatio,
    q_size: Option<u32>,
    r_size: Option<u32>,
    s_size: Option<u32>,
    carryover: Option<usize>,
    trim_larger: Option<bool>,
    allow_fractions: Option<bool>,
) -> Option<GeneratedNumber> {
    generate_number_pattern_beam(
        target.into(),
        Bounds::new(q_size.unwrap_or(8), r_size.unwrap_or(8), s_size.unwrap_or(8)),
        carryover.unwrap_or(25),
        trim_larger.unwrap_or(true),
        allow_fractions.unwrap_or(false),
    )
}

#[pyfunction]
#[pyo3(name = "generate_number_pattern_astar")]
fn generate_number_pattern_astar_py(
    target: PyRatio,
    trim_larger: Option<bool>,
    allow_fractions: Option<bool>,
) -> Option<GeneratedNumber> {
    generate_number_pattern_astar(target.into(), trim_larger.unwrap_or(true), allow_fractions.unwrap_or(false))
}

#[pymodule]
fn hexnumgen(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_number_pattern_beam_py, m)?)?;
    m.add_function(wrap_pyfunction!(generate_number_pattern_astar_py, m)?)?;
    m.add_class::<GeneratedNumber>()?;
    Ok(())
}
