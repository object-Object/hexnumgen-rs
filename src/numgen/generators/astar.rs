use clap::Args;
use num_rational::Ratio;
use pyo3::prelude::*;

use crate::{
    numgen::{Path, PathLimits, QueuedPath},
    utils::NonZeroSign,
};

use std::collections::BinaryHeap;

use super::traits::{AStar, PathGenerator};

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Args)]
pub struct AStarOptions {}

#[pymethods]
impl AStarOptions {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

pub struct AStarPathGenerator {
    // params
    limits: PathLimits,

    // state
    smallest: Option<Path>,
    frontier: BinaryHeap<QueuedPath>,
}

impl PathGenerator for AStarPathGenerator {
    type Opts = AStarOptions;

    fn new(target: Ratio<i64>, trim_larger: bool, allow_fractions: bool, _: AStarOptions) -> Self {
        let mut gen = Self {
            limits: PathLimits::unbounded(target, trim_larger, allow_fractions),
            smallest: None,
            frontier: BinaryHeap::new(),
        };
        gen.push_path(Path::zero(NonZeroSign::from(target)));
        gen
    }

    fn run(self) -> Option<Path> {
        AStar::run(self)
    }
}

impl AStar for AStarPathGenerator {
    fn limits(&self) -> PathLimits {
        self.limits
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
