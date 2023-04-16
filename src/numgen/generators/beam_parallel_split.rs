use std::{sync::Arc, thread};

use crate::{
    numgen::{Bounds, Path, PathLimits, SharedPath},
    traits::RwLockWriteIf,
    utils::{drain_every_other, CondvarAny},
};
use clap::Args;
use num_rational::Ratio;
use parking_lot::RwLock;
use pyo3::prelude::*;

use super::{BeamSearch, PathGenerator};

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Args)]
pub struct BeamSplitOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
    pub num_threads: usize,
}

#[pymethods]
impl BeamSplitOptions {
    #[new]
    fn new(bounds: Bounds, carryover: usize, num_threads: usize) -> Self {
        Self { bounds, carryover, num_threads }
    }
}

pub struct BeamParallelSplitPathGenerator {
    // params
    limits: PathLimits,
    carryover: usize,

    // state
    smallest: SharedPath,
    paths: Vec<Path>,
    num_threads: usize,
    free_threads: Arc<(RwLock<usize>, CondvarAny)>,
}

impl PathGenerator for BeamParallelSplitPathGenerator {
    type Opts = BeamSplitOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        Self::Opts { bounds, carryover, num_threads }: Self::Opts,
    ) -> Self {
        Self {
            limits: PathLimits::bounded(target, trim_larger, allow_fractions, bounds),
            carryover,
            smallest: SharedPath::default(),
            paths: vec![Path::zero(target.into())],
            num_threads,
            free_threads: Arc::new((RwLock::new(num_threads - 1), CondvarAny::default())),
        }
    }
}

impl BeamParallelSplitPathGenerator {
    fn wait_until_done(&self) {
        let mut free_threads = self.free_threads.0.read();
        while *free_threads < self.num_threads {
            self.free_threads.1.wait(&mut free_threads);
        }
    }

    fn split(&mut self) {
        if self.paths.len() > 1 {
            if let Some(mut free_lock) = self.free_threads.0.write_if(|&f| f > 0) {
                *free_lock -= 1;

                // move half of this generator's paths into another thread
                // take every other path so the workload is a bit more even (paths are sorted by a heuristic)
                // this happens fairly infrequently, at least with a large beam size, so it can be somewhat expensive
                let mut child_gen = Self {
                    paths: drain_every_other(&mut self.paths),
                    free_threads: self.free_threads.clone(),
                    smallest: self.smallest.clone(),
                    ..*self
                };
                thread::spawn(move || child_gen.do_search());
            }
        }
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
        while !self.paths.is_empty() {
            self.expand();
            self.trim_to_best();
            self.update_smallest();
            self.split();
        }
        *self.free_threads.0.write() += 1;
        self.free_threads.1.c.notify_one();
    }

    fn get_result(self) -> Option<Path> {
        self.wait_until_done();
        self.smallest.read().clone()
    }
}
