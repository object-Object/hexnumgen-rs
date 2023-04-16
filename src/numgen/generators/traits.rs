use itertools::Itertools;
use num_rational::Ratio;
use num_traits::Zero;
use strum::IntoEnumIterator;

use crate::{
    hex_math::Angle,
    numgen::{path::PathLimits, Path, SharedPath},
    traits::{AbsDiffRatio, RwLockWriteIf},
};

// TODO: fix these types, again
pub trait PathGeneratorRun {
    fn run(self) -> Option<Path>;
}

pub trait PathGenerator: PathGeneratorRun {
    type Opts;

    fn new(target: Ratio<i64>, trim_larger: bool, allow_fractions: bool, opts: Self::Opts) -> Self
    where
        Self: Sized;
}

pub trait BeamSearch {
    fn limits(&self) -> PathLimits;
    fn carryover(&self) -> usize;
    fn smallest(&self) -> &SharedPath;
    fn paths(&self) -> &Vec<Path>;
    fn paths_mut(&mut self) -> &mut Vec<Path>;

    fn target(&self) -> Ratio<u64> {
        self.limits().target
    }

    fn do_search(&mut self) {
        while !self.paths().is_empty() {
            self.expand();
            self.trim_to_best();
            self.update_smallest();
        }
    }

    fn get_result(self) -> Option<Path>
    where
        Self: std::marker::Sized,
    {
        self.smallest().write().take()
    }

    fn expand(&mut self) {
        *self.paths_mut() = self
            .paths()
            .iter()
            .cartesian_product(Angle::iter())
            .filter_map(|(p, a)| p.try_with_angle(a, self.limits(), self.smallest()).ok())
            .collect();
    }

    fn filter_by_key<F, K>(&mut self, new_paths: &mut Vec<Path>, f: F)
    where
        F: FnMut(&Path) -> K,
        K: Ord,
    {
        // put the best values at the start of paths
        new_paths.sort_unstable_by_key(f);

        let carryover = self.carryover();
        if new_paths.len() <= carryover {
            // just move everything out of new_paths
            self.paths_mut().append(new_paths);
        } else {
            // move the first self.carryover paths from new_paths to self.paths
            self.paths_mut().extend(new_paths.drain(..carryover));
        }
    }

    fn trim_to_best(&mut self) {
        let mut rest: Vec<_> = self.paths_mut().drain(..).collect();
        let target = self.target();

        self.filter_by_key(&mut rest, |path| path.len()); // shortest
        self.filter_by_key(&mut rest, |path| path.value().abs_diff(target)); // closest to target
        self.filter_by_key(&mut rest, |path| path.num_points()); // fewest points
    }

    fn update_smallest(&mut self) {
        // appease the borrow checker
        let target = self.target();
        let smallest = self.smallest().clone();

        // prune completed paths from the beam
        self.paths_mut().retain(|path| {
            // if it's not a valid result, just leave it in the beam
            if path.value() != target {
                return true;
            }

            // if it's a valid result, only acquire the write lock if it's better than the current smallest value
            if let Some(mut smallest_lock) = smallest.write_if(|s| path.should_replace(s)) {
                *smallest_lock = Some(path.clone());
            }
            false // don't keep expanding paths that already reached the target
        });
    }
}

impl<T: BeamSearch> PathGeneratorRun for T {
    fn run(mut self) -> Option<Path>
    where
        Self: std::marker::Sized,
    {
        if self.target().is_zero() {
            return self.paths().first().cloned();
        }

        self.do_search();
        self.get_result()
    }
}
