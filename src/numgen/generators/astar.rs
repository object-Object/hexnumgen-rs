use std::{
    collections::BinaryHeap,
    time::{Duration, Instant},
};

use clap::Args;
use num_rational::Ratio;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use super::traits::{AStar, PathGenerator};
use crate::{
    numgen::{Path, PathLimits, QueuedPath},
    utils::NonZeroSign,
};

#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Clone, Copy, Args)]
pub struct AStarOptions {
    #[clap(skip)]
    pub timeout: Option<Duration>,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl AStarOptions {
    #[cfg(feature = "pyo3")]
    #[new]
    #[pyo3(signature = (timeout=None))]
    fn new(timeout: Option<Duration>) -> Self {
        Self { timeout }
    }
}

pub struct AStarPathGenerator {
    // params
    limits: PathLimits,
    deadline: Option<Instant>,

    // state
    smallest: Option<Path>,
    frontier: BinaryHeap<QueuedPath>,
}

impl PathGenerator for AStarPathGenerator {
    type Opts = AStarOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        opts: AStarOptions,
    ) -> Self {
        let mut generator = Self {
            limits: PathLimits::unbounded(target, trim_larger, allow_fractions),
            deadline: opts.timeout.map(|v| Instant::now() + v),
            smallest: None,
            frontier: BinaryHeap::new(),
        };
        generator.push_path(Path::zero(NonZeroSign::from(target)));
        generator
    }

    fn run(self) -> Option<Path> {
        AStar::run(self)
    }
}

impl AStar for AStarPathGenerator {
    fn limits(&self) -> PathLimits {
        self.limits
    }

    fn deadline(&self) -> Option<Instant> {
        self.deadline
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
}
