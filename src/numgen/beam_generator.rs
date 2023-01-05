use std::{cmp::Ordering, mem};

use crate::{hex_math::Angle, utils::NonZeroSign};
use itertools::Itertools;
use strum::IntoEnumIterator;

use super::{Bounds, Path};

pub struct BeamPathGenerator {
    target: i32,
    bounds: Bounds,
    carryover: usize,
    trim_larger: bool,
    smallest: Option<Path>,
    paths: Vec<Path>,
}

impl BeamPathGenerator {
    pub fn new(target: i32, bounds: Bounds, carryover: usize, trim_larger: bool) -> Self {
        Self {
            target: target.abs(),
            bounds,
            carryover,
            trim_larger,
            smallest: None,
            paths: vec![Path::zero(NonZeroSign::from(target))],
        }
    }

    pub fn run(mut self) -> Option<Path> {
        if self.target == 0 {
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
                    if new_path.bounds().fits_in(self.bounds) && (!self.trim_larger || new_path.value() <= self.target)
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
        self.filter_by_key(&mut rest, |path| (path.value() - target).abs()); // closest to target
        self.filter_by_key(&mut rest, |path| path.num_points()); // fewest points
    }

    fn update_smallest(&mut self) {
        let mut rest = Vec::new();
        mem::swap(&mut self.paths, &mut rest);

        for path in rest {
            match path.value().cmp(&self.target) {
                Ordering::Less => self.paths.push(path),
                Ordering::Equal => {
                    if self
                        .smallest
                        .as_ref()
                        .filter(|smallest| !path.bounds().is_better_than(smallest.bounds()))
                        .is_none()
                    {
                        self.smallest = Some(path);
                    }
                }
                _ => (),
            }
        }
    }
}
