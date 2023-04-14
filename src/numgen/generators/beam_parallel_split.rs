use std::{sync::Arc, thread};

use crate::{
    hex_math::Angle,
    numgen::{Bounds, Path},
    traits::{AbsDiffRatio, RwLockWriteIf, UnsignedAbsRatio},
    utils::{drain_every_other, CondvarAny},
};
use itertools::Itertools;
use num_rational::Ratio;
use num_traits::Zero;
use parking_lot::RwLock;
use strum::IntoEnumIterator;

pub struct BeamParallelSplitPathGenerator {
    // params
    target: Ratio<u64>,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
    allow_fractions: bool,

    // state
    num_threads: usize,
    free_threads: Arc<(RwLock<usize>, CondvarAny)>,
    smallest: Arc<RwLock<Option<Path>>>,
    paths: Vec<Path>,
}

impl BeamParallelSplitPathGenerator {
    pub fn new(
        target: Ratio<i64>,
        bounds: Bounds,
        carryover: usize,
        trim_larger: bool,
        allow_fractions: bool,
        num_threads: usize,
    ) -> Self {
        Self {
            target: target.unsigned_abs(),
            bounds,
            carryover,
            trim_larger,
            allow_fractions,
            num_threads,
            free_threads: Arc::new((RwLock::new(num_threads - 1), CondvarAny::default())),
            smallest: Arc::default(),
            paths: vec![Path::zero(target.into())],
        }
    }

    pub fn run(mut self) -> Option<Path> {
        if self.target.is_zero() {
            return Some(self.paths[0].clone());
        }

        // start the search, then wait for all threads to finish before returning the result
        self.do_search();
        self.wait_until_done();
        self.smallest.read().clone()
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

    fn wait_until_done(&self) {
        let mut free_threads = self.free_threads.0.read();
        while *free_threads < self.num_threads {
            self.free_threads.1.wait(&mut free_threads);
        }
    }

    fn split(&mut self) {
        // TODO: tweak cutoff for performance
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

    fn expand(&mut self) {
        self.paths = self
            .paths
            .iter()
            .cartesian_product(Angle::iter())
            .filter_map(|(path, angle)| {
                if let Ok(new_path) = path.with_angle(angle) {
                    if (!self.trim_larger || new_path.value() <= self.target)
                        && (self.allow_fractions || new_path.value().is_integer())
                        && new_path.bounds().fits_in(self.bounds)
                        && new_path.should_replace(&self.smallest.read())
                    {
                        return Some(new_path);
                    }
                }
                None
            })
            .collect();
    }

    fn filter_by_key<F, K>(&mut self, new_paths: &mut Vec<Path>, f: F)
    where
        F: FnMut(&Path) -> K,
        K: Ord,
    {
        // put the best values at the start of paths
        new_paths.sort_unstable_by_key(f);

        if new_paths.len() <= self.carryover {
            // just move everything out of new_paths
            self.paths.append(new_paths);
        } else {
            // move the first self.carryover paths from new_paths to self.paths
            self.paths.extend(new_paths.drain(..self.carryover));
        }
    }

    fn trim_to_best(&mut self) {
        let mut rest: Vec<_> = self.paths.drain(..).collect();
        let target = self.target;

        self.filter_by_key(&mut rest, |path| path.len()); // shortest
        self.filter_by_key(&mut rest, |path| path.value().abs_diff(target)); // closest to target
        self.filter_by_key(&mut rest, |path| path.num_points()); // fewest points
    }

    fn update_smallest(&mut self) {
        self.paths.retain(|path| {
            // if it's not a valid result, just leave it in the beam
            if path.value() != self.target {
                return true;
            }

            if let Some(mut smallest_lock) = self.smallest.write_if(|s| path.should_replace(s)) {
                *smallest_lock = Some(path.clone());
            }
            false
        });
    }
}
