use std::{collections::BinaryHeap, time::Instant};

use num_rational::Ratio;
use num_traits::Zero;
use strum::IntoEnumIterator;

use crate::{
    hex_math::Angle,
    numgen::{Path, PathLimits, QueuedPath},
};

pub trait AStar {
    fn limits(&self) -> PathLimits;
    fn deadline(&self) -> Option<Instant>;
    fn smallest(&self) -> &Option<Path>;
    fn smallest_mut(&mut self) -> &mut Option<Path>;
    fn frontier(&self) -> &BinaryHeap<QueuedPath>;
    fn frontier_mut(&mut self) -> &mut BinaryHeap<QueuedPath>;

    fn target(&self) -> Ratio<u64> {
        self.limits().target
    }

    fn run(mut self) -> Option<Path>
    where
        Self: Sized,
    {
        if self.target().is_zero() {
            return self.frontier_mut().pop().map(Into::into);
        }

        self.do_search();
        self.get_result()
    }

    fn do_search(&mut self) {
        while !self.frontier().is_empty()
            && self
                .deadline()
                .is_none_or(|deadline| deadline > Instant::now())
        {
            if self.update_frontier()
                && let Some(new_smallest) = self.find_best_in_frontier()
                && new_smallest.should_replace(self.smallest())
            {
                self.update_smallest_and_prune(new_smallest.clone());
            }
        }
    }

    fn get_result(self) -> Option<Path>
    where
        Self: Sized,
    {
        self.smallest().clone()
    }

    fn find_best_in_frontier(&self) -> Option<&Path> {
        self.frontier()
            .iter()
            .map(|qp| &qp.path)
            .filter(|path| path.value() == self.target())
            .min_by_key(|path| (path.bounds().quasi_area(), path.len()))
    }

    fn push_path(&mut self, path: Path) {
        let priority = self.heuristic(&path);
        self.frontier_mut().push(QueuedPath { path, priority });
    }

    fn pop_path(&mut self) -> Option<Path> {
        self.frontier_mut().pop().map(|qp| qp.path)
    }

    fn next_paths(&self, path: Path) -> Vec<Path> {
        Angle::iter()
            .filter_map(|a| {
                path.try_with_angle(a, self.limits(), |n| n.should_replace(self.smallest()))
                    .ok()
            })
            .collect()
    }

    /// Returns true if there are valid solutions in the new frontier
    fn update_frontier(&mut self) -> bool {
        let path = self.pop_path();
        let mut has_valid_solutions = false;

        if let Some(path) = path {
            for new_path in self.next_paths(path) {
                if new_path.value() == self.target() {
                    has_valid_solutions = true;
                }
                self.push_path(new_path);
            }
        }

        has_valid_solutions
    }

    fn update_smallest_and_prune(&mut self, new_smallest: Path) {
        let new_smallest = Some(new_smallest);
        self.frontier_mut()
            .retain(|qp| qp.path.should_replace(&new_smallest));
        *self.smallest_mut() = new_smallest;
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
}
