use clap::Args;
use num_rational::Ratio;
use num_traits::Zero;
use parking_lot::RwLock;
use pyo3::prelude::*;
use std::{mem, sync::Arc};
use strum::IntoEnumIterator;

use crate::{hex_math::Angle, numgen::Path, threadpool::ThreadPool, traits::UnsignedAbsRatio, Bounds};

use super::{BeamOptions, BeamPathGenerator, PathGenerator};

#[pyclass]
#[derive(Clone, Copy, Args)]
pub struct BeamPoolOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
    pub num_threads: usize,
}

#[pymethods]
impl BeamPoolOptions {
    #[new]
    fn new(bounds: Bounds, carryover: usize, num_threads: usize) -> Self {
        Self { bounds, carryover, num_threads }
    }
}

impl From<BeamPoolOptions> for BeamOptions {
    fn from(BeamPoolOptions { bounds, carryover, .. }: BeamPoolOptions) -> Self {
        Self { bounds, carryover }
    }
}

pub struct BeamParallelPoolPathGenerator {
    inner: BeamPathGenerator,
    pool: ThreadPool<Path, Vec<Path>>,
    pool_smallest: Arc<RwLock<Option<Path>>>,
}

impl PathGenerator for BeamParallelPoolPathGenerator {
    type Opts = BeamPoolOptions;

    fn new(target: Ratio<i64>, trim_larger: bool, allow_fractions: bool, opts: Self::Opts) -> Self {
        let pool_smallest = Arc::new(RwLock::new(None));
        let Self::Opts { bounds, num_threads, .. } = opts;

        let pool = {
            let pool_smallest = pool_smallest.clone();
            let target = target.unsigned_abs();

            ThreadPool::new(num_threads, move |path: Path| {
                Angle::iter()
                    .filter_map(|angle| {
                        if let Ok(new_path) = path.with_angle(angle) {
                            if (!trim_larger || new_path.value() <= target)
                                && (allow_fractions || new_path.value().is_integer())
                                && new_path.bounds().fits_in(bounds)
                                && new_path.should_replace(&pool_smallest.read())
                            {
                                return Some(new_path);
                            }
                        }
                        None
                    })
                    .collect()
            })
        };

        Self { inner: BeamPathGenerator::new(target, trim_larger, allow_fractions, opts.into()), pool, pool_smallest }
    }

    fn run(mut self) -> Option<Path> {
        if self.inner.target.is_zero() {
            return Some(self.inner.paths[0].clone());
        }
        while !self.inner.paths.is_empty() {
            self.expand();
            self.inner.trim_to_best();
            self.inner.update_smallest();
            *self.pool_smallest.write() = self.inner.smallest.clone();
        }
        self.inner.smallest
    }
}

impl BeamParallelPoolPathGenerator {
    pub fn expand(&mut self) {
        let old_paths = mem::take(&mut self.inner.paths);
        self.inner.paths = self.pool.map(old_paths).into_iter().flatten().collect();
    }
}
