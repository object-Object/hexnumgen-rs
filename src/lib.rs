#[macro_use]
extern crate lazy_static;

mod drawing;
mod errors;
mod hex_math;
mod numgen;
mod threadpool;
mod traits;
mod utils;

use clap::Subcommand;
pub use drawing::{PatternPlotter, pattern_to_points};
pub use hex_math::Direction;
use num_rational::Ratio;
pub use numgen::{
    Bounds,
    generators::{AStarOptions, AStarSplitOptions, BeamOptions, BeamPoolOptions, BeamSplitOptions},
};
use numgen::{
    Path,
    generators::{
        AStarParallelSplitPathGenerator, AStarPathGenerator, BeamParallelPoolPathGenerator,
        BeamParallelSplitPathGenerator, BeamPathGenerator, traits::PathGenerator,
    },
};
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub enum PyRatio {
    #[cfg_attr(feature = "pyo3", pyo3(annotation = "int"))]
    Int(i64),
    #[cfg_attr(feature = "pyo3", pyo3(annotation = "tuple[int, int]"))]
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

#[derive(Subcommand)]
#[cfg_attr(feature = "pyo3", derive(FromPyObject))]
pub enum GeneratorOptions {
    Beam(BeamOptions),
    BeamPool(BeamPoolOptions),
    BeamSplit(BeamSplitOptions),
    #[command(name = "astar")]
    AStar(AStarOptions),
    #[command(name = "astar-split")]
    AStarSplit(AStarSplitOptions),
}

#[cfg_attr(feature = "pyo3", pyclass(get_all))]
pub struct GeneratedNumber {
    pub direction: String,
    pub pattern: String,
    pub bounds: Bounds,
    pub num_points: usize,
    pub num_segments: usize,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl GeneratedNumber {
    fn __str__(&self) -> String {
        format!("{} {}", self.direction, self.pattern)
    }
}

impl From<Path> for GeneratedNumber {
    fn from(path: Path) -> Self {
        Self {
            direction: path.starting_direction().to_string(),
            pattern: path.pattern(),
            bounds: path.bounds(),
            num_points: path.num_points(),
            num_segments: path.len(),
        }
    }
}

pub fn generate_number_pattern(
    target: Ratio<i64>,
    trim_larger: bool,
    allow_fractions: bool,
    options: GeneratorOptions,
) -> Option<GeneratedNumber> {
    // TODO: fix these types. ew
    match options {
        GeneratorOptions::Beam(opts) => {
            BeamPathGenerator::new(target, trim_larger, allow_fractions, opts).run()
        }
        GeneratorOptions::BeamPool(opts) => {
            BeamParallelPoolPathGenerator::new(target, trim_larger, allow_fractions, opts).run()
        }
        GeneratorOptions::BeamSplit(opts) => {
            BeamParallelSplitPathGenerator::new(target, trim_larger, allow_fractions, opts).run()
        }
        GeneratorOptions::AStar(opts) => {
            AStarPathGenerator::new(target, trim_larger, allow_fractions, opts).run()
        }
        GeneratorOptions::AStarSplit(opts) => {
            AStarParallelSplitPathGenerator::new(target, trim_larger, allow_fractions, opts).run()
        }
    }
    .map(Into::into)
}

#[cfg(feature = "pyo3")]
#[pyfunction]
#[pyo3(name = "generate_number_pattern")]
fn generate_number_pattern_py(
    target: PyRatio,
    trim_larger: bool,
    allow_fractions: bool,
    options: GeneratorOptions,
) -> Option<GeneratedNumber> {
    generate_number_pattern(target.into(), trim_larger, allow_fractions, options)
}

#[cfg(feature = "pyo3")]
#[pymodule]
fn hexnumgen(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_number_pattern_py, m)?)?;
    m.add_class::<GeneratedNumber>()?;
    m.add_class::<Bounds>()?;
    m.add_class::<BeamOptions>()?;
    m.add_class::<BeamPoolOptions>()?;
    m.add_class::<BeamSplitOptions>()?;
    m.add_class::<AStarOptions>()?;
    m.add_class::<AStarSplitOptions>()?;
    Ok(())
}
