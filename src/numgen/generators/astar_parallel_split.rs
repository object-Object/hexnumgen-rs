use std::{
    collections::BinaryHeap,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Instant,
};

use clap::Args;
use itertools::Itertools;
use num_rational::Ratio;
use parking_lot::{Condvar, Mutex, RwLock};
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use super::traits::{AStar, PathGenerator, Split};
use crate::{
    numgen::{Path, PathLimits, QueuedPath, SharedPath},
    traits::RwLockWriteIf,
    utils::NonZeroSign,
};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Clone, Copy, Args)]
pub struct AStarSplitOptions {
    pub num_threads: usize,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl AStarSplitOptions {
    #[cfg(feature = "pyo3")]
    #[new]
    fn new(num_threads: usize) -> Self {
        Self { num_threads }
    }
}

pub struct AStarParallelSplitPathGenerator {
    // params
    limits: PathLimits,
    num_threads: usize,

    // state
    frontier: BinaryHeap<QueuedPath>,
    smallest: Option<Path>,
    shared_smallest: SharedPath,
    free_threads: Arc<RwLock<usize>>,
    done: Arc<(Mutex<bool>, Condvar)>,
}

impl AStarParallelSplitPathGenerator {
    fn bisect_frontier(&mut self) -> BinaryHeap<QueuedPath> {
        // remove half of the paths from the current thread's frontier in no particular order
        let mut paths = self.frontier.drain().collect_vec();
        self.frontier.extend(paths.drain((paths.len() / 2)..));
        paths.into()
    }
}

impl PathGenerator for AStarParallelSplitPathGenerator {
    type Opts = AStarSplitOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        AStarSplitOptions { num_threads }: AStarSplitOptions,
    ) -> Self {
        let mut generator = Self {
            limits: PathLimits::unbounded(target, trim_larger, allow_fractions),
            num_threads,
            smallest: None,
            frontier: BinaryHeap::new(),
            shared_smallest: SharedPath::default(),
            free_threads: Arc::new(RwLock::new(num_threads - 1)),
            done: Arc::new((false.into(), Condvar::new())),
        };
        generator.push_path(Path::zero(NonZeroSign::from(target)));
        generator
    }

    fn run(self) -> Option<Path> {
        AStar::run(self)
    }
}

impl Split for AStarParallelSplitPathGenerator {
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
        // create and run the child thread
        let mut child_gen = Self {
            frontier: self.bisect_frontier(),
            smallest: self.smallest.clone(),
            shared_smallest: self.shared_smallest.clone(),
            free_threads: self.free_threads.clone(),
            done: self.done.clone(),
            ..*self
        };
        thread::spawn(move || child_gen.do_search())
    }
}

impl AStar for AStarParallelSplitPathGenerator {
    fn limits(&self) -> PathLimits {
        self.limits
    }

    fn deadline(&self) -> Option<Instant> {
        None
    }

    fn smallest(&self) -> &Option<Path> {
        &self.smallest
    }

    fn smallest_mut(&mut self) -> &mut Option<Path> {
        &mut self.smallest
    }

    fn frontier(&self) -> &BinaryHeap<QueuedPath> {
        &self.frontier
    }

    fn frontier_mut(&mut self) -> &mut BinaryHeap<QueuedPath> {
        &mut self.frontier
    }

    fn do_search(&mut self) {
        while !self.frontier().is_empty() {
            // check if another thread has found a better result, and update ours if so
            let shared_smallest = match &*self.shared_smallest.read() {
                Some(s) if s.should_replace(&self.smallest) => Some(s.clone()),
                _ => None,
            };
            if let Some(new_smallest) = shared_smallest {
                self.update_smallest_and_prune(new_smallest);
            }

            // main algorithm
            // if the above section emptied the frontier, update_frontier() just returns false, so this is skipped
            if self.update_frontier()
                && let Some(new_smallest) = self.find_best_in_frontier()
                && new_smallest.should_replace(self.smallest())
            {
                let new_smallest = new_smallest.clone();

                // if we found a better result than the shared one, update the shared one
                if let Some(mut lock) = self
                    .shared_smallest
                    .write_if(|s| new_smallest.should_replace(s))
                {
                    *lock = Some(new_smallest.clone());
                }

                self.update_smallest_and_prune(new_smallest);
            }

            // spawn a child if possible
            if self.frontier.len() > 1 {
                self.split();
            }
        }
        self.merge()
    }

    fn get_result(self) -> Option<Path>
    where
        Self: Sized,
    {
        self.wait_until_done();
        self.shared_smallest.write().take()
    }
}
