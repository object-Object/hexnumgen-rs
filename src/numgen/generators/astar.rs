use clap::Args;
use num_rational::Ratio;
use num_traits::Zero;
use pyo3::prelude::*;
use strum::IntoEnumIterator;

use crate::{
    hex_math::Angle,
    numgen::{Path, PathLimits, QueuedPath, SharedPath},
    traits::RwLockWriteIf,
    utils::NonZeroSign,
};

use std::collections::BinaryHeap;

use super::{traits::PathGeneratorRun, PathGenerator};

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
    smallest: SharedPath,
    frontier: BinaryHeap<QueuedPath>,
}

impl PathGenerator for AStarPathGenerator {
    type Opts = AStarOptions;

    fn new(target: Ratio<i64>, trim_larger: bool, allow_fractions: bool, _: AStarOptions) -> Self {
        let mut gen = Self {
            limits: PathLimits::unbounded(target, trim_larger, allow_fractions),
            smallest: SharedPath::default(),
            frontier: BinaryHeap::new(),
        };
        gen.push_path(Path::zero(NonZeroSign::from(target)));
        gen
    }
}

impl PathGeneratorRun for AStarPathGenerator {
    fn run(mut self) -> Option<Path> {
        if self.target().is_zero() {
            return self.frontier.pop().map(Into::into);
        }

        self.do_search();
        self.get_result()
    }
}

impl AStarPathGenerator {
    fn target(&self) -> Ratio<u64> {
        self.limits.target
    }

    fn do_search(&mut self) {
        while !self.frontier.is_empty() {
            // formatting note: https://github.com/rust-lang/rustfmt/pull/5203
            if self.update_frontier()
                && let Some(new_smallest) = self.find_best_in_frontier()
                && let Some(mut smallest_lock) = self.smallest.write_if(|s| new_smallest.should_replace(s))
            {
                let opt = Some(new_smallest.clone());
                self.frontier.retain(|qp| qp.path.should_replace(&opt));
                *smallest_lock = opt;
            }
        }
    }

    fn get_result(self) -> Option<Path> {
        self.smallest.write().take()
    }

    fn find_best_in_frontier(&self) -> Option<&Path> {
        self.frontier
            .iter()
            .map(|qp| &qp.path)
            .filter(|path| path.value() == self.target())
            .min_by_key(|path| path.bounds().quasi_area())
    }

    /// Returns true if there are valid solutions in the new frontier
    fn update_frontier(&mut self) -> bool {
        let path = self.frontier.pop().unwrap().path;
        let mut has_valid_solutions = false;

        for new_path in self.next_paths(path) {
            if new_path.value() == self.target() {
                has_valid_solutions = true;
            }
            self.push_path(new_path);
        }

        has_valid_solutions
    }

    fn next_paths(&self, path: Path) -> Vec<Path> {
        Angle::iter().filter_map(|a| path.try_with_angle(a, self.limits, &self.smallest).ok()).collect()
    }

    fn heuristic(&mut self, path: &Path) -> usize {
        let mut val = path.value();
        let mut target = self.target();
        let mut heuristic = path.len();

        if val.is_zero() {
            heuristic += 1;

            if target > 10.into() {
                val += 10;
            } else if target > 5.into() {
                val += 5;
            } else {
                val += 1;
            }
        }

        if !target.is_zero() {
            while val > target {
                val /= 2;
                heuristic += 1;
            }

            while target / 2 > val {
                target /= 2;
                heuristic += 1;
            }
        }

        heuristic
    }

    fn push_path(&mut self, path: Path) {
        let priority = self.heuristic(&path);
        self.frontier.push(QueuedPath { path, priority });
    }
}
