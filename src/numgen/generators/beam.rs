use crate::{
    hex_math::Angle,
    numgen::{Bounds, Path},
    traits::{AbsDiffRatio, UnsignedAbsRatio},
};
use clap::Args;
use itertools::Itertools;
use num_rational::Ratio;
use num_traits::Zero;
use pyo3::prelude::*;
use strum::IntoEnumIterator;

use super::PathGenerator;

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Args)]
pub struct BeamOptions {
    #[command(flatten)]
    pub bounds: Bounds,
    #[arg(short, long, default_value_t = 25)]
    pub carryover: usize,
}

#[pymethods]
impl BeamOptions {
    #[new]
    fn new(bounds: Bounds, carryover: usize) -> Self {
        Self { bounds, carryover }
    }
}

pub struct BeamPathGenerator {
    // params
    pub target: Ratio<u64>,
    pub trim_larger: bool,
    pub allow_fractions: bool,
    pub bounds: Bounds,
    pub carryover: usize,

    // state
    pub smallest: Option<Path>,
    pub paths: Vec<Path>,
}

impl PathGenerator for BeamPathGenerator {
    type Opts = BeamOptions;

    fn new(
        target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        Self::Opts { bounds, carryover }: Self::Opts,
    ) -> Self {
        Self {
            target: target.unsigned_abs(),
            bounds,
            carryover,
            trim_larger,
            allow_fractions,
            smallest: None,
            paths: vec![Path::zero(target.into())],
        }
    }

    fn run(mut self) -> Option<Path> {
        if self.target.is_zero() {
            return Some(self.paths[0].clone());
        }
        while !self.paths.is_empty() {
            self.expand();
            self.trim_to_best();
            self.update_smallest();
        }
        self.smallest
    }
}

impl BeamPathGenerator {
    pub fn expand(&mut self) {
        self.paths = self
            .paths
            .iter()
            .cartesian_product(Angle::iter())
            .filter_map(|(path, angle)| {
                if let Ok(new_path) = path.with_angle(angle) {
                    if (!self.trim_larger || new_path.value() <= self.target)
                        && (self.allow_fractions || new_path.value().is_integer())
                        && new_path.bounds().fits_in(self.bounds)
                        && new_path.should_replace(&self.smallest)
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

    pub fn trim_to_best(&mut self) {
        let mut rest: Vec<_> = self.paths.drain(..).collect();
        let target = self.target;

        self.filter_by_key(&mut rest, |path| path.len()); // shortest
        self.filter_by_key(&mut rest, |path| path.value().abs_diff(target)); // closest to target
        self.filter_by_key(&mut rest, |path| path.num_points()); // fewest points
    }

    pub fn update_smallest(&mut self) {
        self.paths.retain(|path| {
            if path.value() != self.target {
                return true;
            }
            if path.should_replace(&self.smallest) {
                // even if it's not optimized away, this clone should be fine because it's very infrequent
                self.smallest = Some(path.clone());
            }
            false
        });
    }
}
