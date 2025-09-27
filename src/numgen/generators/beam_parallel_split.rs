use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use clap::Args;
use num_rational::Ratio;
use parking_lot::{Condvar, Mutex, RwLock};
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use super::traits::{BeamSearch, PathGenerator, Split};
use crate::{
    numgen::{Bounds, Path, PathLimits, SharedPath},
    utils::drain_every_other,
};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Clone, Copy, Args)]
pub struct BeamSplitOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
    pub num_threads: usize,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl BeamSplitOptions {
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

pub struct BeamParallelSplitPathGenerator {
    // params
    limits: PathLimits,
    carryover: usize,
    num_threads: usize,

    // state
    paths: Vec<Path>,
    smallest: SharedPath,
    free_threads: Arc<RwLock<usize>>,
    done: Arc<(Mutex<bool>, Condvar)>,
}

impl PathGenerator for BeamParallelSplitPathGenerator {
    type Opts = BeamSplitOptions;

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
        Self {
            limits: PathLimits::bounded(target, trim_larger, allow_fractions, bounds),
            carryover,
            smallest: SharedPath::default(),
            paths: vec![Path::zero(target.into())],
            num_threads,
            free_threads: Arc::new(RwLock::new(num_threads - 1)),
            done: Arc::new((false.into(), Condvar::new())),
        }
    }

    fn run(self) -> Option<Path> {
        BeamSearch::run(self)
    }
}

impl Split for BeamParallelSplitPathGenerator {
    fn num_threads(&self) -> usize {
        self.num_threads
    }

    fn free_threads(&self) -> &Arc<RwLock<usize>> {
        &self.free_threads
    }

    fn done(&self) -> &Arc<(Mutex<bool>, Condvar)> {
        &self.done
    }

    fn spawn_child(&mut self) -> JoinHandle<()> {
        // move half of this generator's paths into another thread
        // take every other path so the workload is a bit more even (paths are sorted by a heuristic)
        // this happens fairly infrequently, at least with a large beam size, so it can be somewhat expensive
        let mut child_gen = Self {
            paths: drain_every_other(&mut self.paths),
            smallest: self.smallest.clone(),
            free_threads: self.free_threads.clone(),
            done: self.done.clone(),
            ..*self
        };
        thread::spawn(move || child_gen.do_search())
    }
}

impl BeamSearch for BeamParallelSplitPathGenerator {
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

    fn do_search(&mut self) {
        // main loop
        while !self.paths.is_empty() {
            self.expand();
            self.trim_to_best();
            self.update_smallest();
            if self.paths.len() > 1 {
                self.split();
            }
        }
        self.merge()
    }

    fn get_result(self) -> Option<Path> {
        self.wait_until_done();
        self.smallest.write().take()
    }
}
