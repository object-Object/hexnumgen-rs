use std::{sync::Arc, thread};

use crate::{
    numgen::{Bounds, Path, PathLimits, SharedPath},
    traits::RwLockWriteIf,
    utils::drain_every_other,
};
use clap::Args;
use num_rational::Ratio;
use parking_lot::{Condvar, Mutex, RwLock};
use pyo3::prelude::*;

use super::traits::{BeamSearch, PathGenerator};

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
        Self::Opts { bounds, carryover, num_threads }: Self::Opts,
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

impl BeamParallelSplitPathGenerator {
    fn wait_until_done(&self) {
        // wait for the condition variable to trigger
        let mut lock = self.done.0.lock();
        self.done.1.wait_while(&mut lock, |done| !*done);
    }

    fn split(&mut self) {
        if self.paths.len() > 1 {
            // acquire a write lock if there's space for more threads to be created
            if let Some(mut free_lock) = self.free_threads.write_if(|&f| f > 0) {
                // decrement the number of free threads, then immediately release the lock so other threads can stop waiting
                *free_lock -= 1;
                drop(free_lock);

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
        // main loop
        while !self.paths.is_empty() {
            self.expand();
            self.trim_to_best();
            self.update_smallest();
            self.split();
        }

        // no more work to do on this thread, so increment free_threads
        // do the increment in a block so the lock is released before acquiring the condvar mutex, to avoid deadlocks
        let free_threads = {
            let mut lock = self.free_threads.write();
            *lock += 1;
            *lock
        };
        if free_threads == self.num_threads {
            // this is the last working thread, so tell the main thread it's safe to exit
            let mut lock = self.done.0.lock();
            *lock = true;
            self.done.1.notify_one();
        }
    }

    fn get_result(self) -> Option<Path> {
        self.wait_until_done();
        self.smallest.write().take()
    }
}
