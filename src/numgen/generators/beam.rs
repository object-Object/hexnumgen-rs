use crate::numgen::{Bounds, Path, PathLimits, SharedPath};
use clap::Args;
use num_rational::Ratio;
use pyo3::prelude::*;

use super::{BeamSearch, PathGenerator};

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Args)]
pub struct BeamOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
}

#[pymethods]
impl BeamOptions {
    #[new]
    fn new(bounds: Bounds, carryover: usize) -> Self {
        Self { bounds, carryover }
    }
}

pub struct BeamPathGenerator {
    // params
    limits: PathLimits,
    carryover: usize,

    // state
    // this generator is sequential, but we use a RwLock here because it simplifies the implementation of the parallel versions
    // parking_lot's RwLock has an inline fast path for uncontended locks, so this should be fine
    smallest: SharedPath,
    paths: Vec<Path>,
}

impl PathGenerator for BeamPathGenerator {
    type Opts = BeamOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        Self::Opts { bounds, carryover }: Self::Opts,
    ) -> Self {
        Self {
            limits: PathLimits::bounded(target, trim_larger, allow_fractions, bounds),
            carryover,
            smallest: SharedPath::default(),
            paths: vec![Path::zero(target.into())],
        }
    }
}

impl BeamSearch for BeamPathGenerator {
    fn limits(&self) -> PathLimits {
        self.limits
    }

    fn carryover(&self) -> usize {
        self.carryover
    }

    fn smallest(&self) -> &SharedPath {
        &self.smallest
    }

    fn paths(&self) -> &Vec<Path> {
        &self.paths
    }

    fn paths_mut(&mut self) -> &mut Vec<Path> {
        &mut self.paths
    }
}
