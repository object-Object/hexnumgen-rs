use clap::Args;
use num_rational::Ratio;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
use strum::IntoEnumIterator;

use super::{
    traits::{BeamSearch, PathGenerator},
    BeamOptions,
};
use crate::{
    hex_math::Angle,
    numgen::{Path, PathLimits, SharedPath},
    threadpool::ThreadPool,
    Bounds,
};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Clone, Copy, Args)]
pub struct BeamPoolOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
    pub num_threads: usize,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl BeamPoolOptions {
    #[cfg(feature = "pyo3")]
    #[new]
    fn new(bounds: Bounds, carryover: usize, num_threads: usize) -> Self {
        Self {
            bounds,
            carryover,
            num_threads,
        }
    }
}

impl From<BeamPoolOptions> for BeamOptions {
    fn from(
        BeamPoolOptions {
            bounds, carryover, ..
        }: BeamPoolOptions,
    ) -> Self {
        Self { bounds, carryover }
    }
}

pub struct BeamParallelPoolPathGenerator {
    // params
    limits: PathLimits,
    carryover: usize,

    // state
    smallest: SharedPath,
    paths: Vec<Path>,
    pool: ThreadPool<Path, Vec<Path>>,
}

impl PathGenerator for BeamParallelPoolPathGenerator {
    type Opts = BeamPoolOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        Self::Opts {
            bounds,
            carryover,
            num_threads,
        }: Self::Opts,
    ) -> Self {
        let limits = PathLimits::bounded(target, trim_larger, allow_fractions, bounds);
        let smallest = SharedPath::default();

        let pool = {
            // make a copy of the rwlock for the threads in the pool to use
            let smallest = smallest.clone();

            ThreadPool::new(num_threads, move |p: Path| {
                Angle::iter()
                    .filter_map(|a| {
                        p.try_with_angle(a, limits, |n| n.should_replace(&smallest.read()))
                            .ok()
                    })
                    .collect()
            })
        };

        Self {
            limits,
            carryover,
            smallest,
            pool,
            paths: vec![Path::zero(target.into())],
        }
    }

    fn run(self) -> Option<Path> {
        BeamSearch::run(self)
    }
}

impl BeamSearch for BeamParallelPoolPathGenerator {
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

    fn expand(&mut self) {
        let old_paths = self.paths.split_off(0);
        self.paths.extend(self.pool.map_args(old_paths).flatten());
    }
}
