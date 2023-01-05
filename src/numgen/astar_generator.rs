use strum::IntoEnumIterator;

use crate::{hex_math::Angle, utils::NonZeroSign};

use super::{Bounds, Path, QueuedPath};
use std::collections::BinaryHeap;

pub struct AStarPathGenerator {
    target: u32,
    bounds: Bounds,
    trim_larger: bool,
    smallest: Option<Path>,
    frontier: BinaryHeap<QueuedPath>,
}

impl AStarPathGenerator {
    pub fn new(target: i32, bounds: Bounds, trim_larger: bool) -> Self {
        let mut gen =
            Self { target: target.unsigned_abs(), bounds, trim_larger, smallest: None, frontier: BinaryHeap::new() };
        gen.push_path(Path::zero(NonZeroSign::from(target)));
        gen
    }

    pub fn run(mut self) -> Option<Path> {
        if self.target == 0 {
            return self.frontier.pop().map(Into::into);
        }

        while !self.frontier.is_empty() {
            // i really wish if-let chains were stable
            if self.update_frontier() {
                if let Some(smallest_in_frontier) = self
                    .frontier
                    .iter()
                    .map(|qp| &qp.path)
                    .filter(|path| path.value() == self.target)
                    .min_by_key(|path| path.bounds().quasi_area())
                {
                    if smallest_in_frontier.should_replace(&self.smallest) {
                        let smallest = smallest_in_frontier.clone();

                        // i really wish BinaryHeap retain was stable
                        self.frontier = BinaryHeap::from_iter(
                            self.frontier.into_iter().filter(|qp| qp.path.bounds().is_better_than(smallest.bounds())),
                        );

                        self.smallest = Some(smallest);
                    }
                }
            }
        }

        self.smallest
    }

    /// Returns true if there are valid solutions in the new frontier
    fn update_frontier(&mut self) -> bool {
        let path = self.frontier.pop().unwrap().path;
        let mut has_valid_solutions = false;

        for new_path in self.next_paths(path) {
            if new_path.value() == self.target {
                has_valid_solutions = true;
            }
            self.push_path(new_path);
        }

        has_valid_solutions
    }

    fn next_paths(&self, path: Path) -> Vec<Path> {
        Angle::iter()
            .filter_map(|angle| {
                if let Ok(new_path) = path.with_angle(angle) {
                    if new_path.bounds().fits_in(self.bounds) && (!self.trim_larger || new_path.value() <= self.target)
                    {
                        return Some(new_path);
                    }
                }
                None
            })
            .collect()
    }

    fn heuristic(&mut self, path: &Path) -> usize {
        let mut val = path.value();
        let mut target = self.target;
        let mut heuristic = path.len();

        if val == 0 {
            heuristic += 1;

            if target > 10 {
                val += 10;
            } else if target > 5 {
                val += 5;
            } else {
                val += 1;
            }
        }

        while val > target {
            val /= 2;
            heuristic += 1;
        }

        while target / 2 > val {
            target /= 2;
            heuristic += 1;
        }

        heuristic
    }

    fn push_path(&mut self, path: Path) {
        let priority = self.heuristic(&path);
        self.frontier.push(QueuedPath { path, priority });
    }
}
