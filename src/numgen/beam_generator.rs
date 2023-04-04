use std::mem;

use crate::{
    hex_math::Angle,
    traits::{AbsDiffRatio, UnsignedAbsRatio},
};
use itertools::Itertools;
use num_rational::Ratio;
use num_traits::Zero;
use strum::IntoEnumIterator;

use super::{Bounds, Path};

pub struct BeamPathGenerator {
    target: Ratio<u64>,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
    allow_fractions: bool,
    smallest: Option<Path>,
    paths: Vec<Path>,
}

impl BeamPathGenerator {
    pub fn new(target: Ratio<i64>, bounds: Bounds, carryover: usize, trim_larger: bool, allow_fractions: bool) -> Self {
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

    pub fn run(mut self) -> Option<Path> {
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
                    {
                        return Some(new_path);
                    }
                }
                None
            })
            .collect();
    }

    fn filter_by_key<F, K>(&mut self, paths: &mut Vec<Path>, f: F)
    where
        F: FnMut(&Path) -> K,
        K: Ord,
    {
        paths.sort_by_key(f);

        if self.carryover > paths.len() {
            self.paths.append(paths);
        } else {
            let rest = paths.split_off(self.carryover);
            self.paths.append(paths);
            paths.extend(rest);
        }
    }

    fn trim_to_best(&mut self) {
        let mut rest = Vec::new();
        mem::swap(&mut rest, &mut self.paths);

        let target = self.target;

        self.filter_by_key(&mut rest, |path| path.len()); // shortest
        self.filter_by_key(&mut rest, |path| path.value().abs_diff(target)); // closest to target
        self.filter_by_key(&mut rest, |path| path.num_points()); // fewest points
    }

    fn update_smallest(&mut self) {
        let mut rest = Vec::new();
        mem::swap(&mut self.paths, &mut rest);

        for path in rest {
            if path.value() != self.target {
                self.paths.push(path);
            } else if path.should_replace(&self.smallest) {
                self.smallest = Some(path);
            }
        }
    }
}
