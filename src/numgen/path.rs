use std::{collections::HashSet, sync::Arc};

use itertools::Itertools;
use num_rational::Ratio;
use parking_lot::RwLock;

use super::{Bounds, MinMax};
use crate::{
    errors::{HexError, HexResult},
    hex_math::{get_pattern_segments, Angle, Coord, Direction, Segment},
    traits::UnsignedAbsRatio,
    utils::{cloned_push, cloned_union_single, NonZeroSign},
};

pub type SharedPath = Arc<RwLock<Option<Path>>>;

#[derive(Clone, Copy)]
pub struct PathLimits {
    pub target: Ratio<u64>,
    pub trim_larger: bool,
    pub allow_fractions: bool,
    pub bounds: Option<Bounds>,
}

impl PathLimits {
    pub fn bounded(
        signed_target: Ratio<i64>,
        trim_larger: bool,
        allow_fractions: bool,
        bounds: Bounds,
    ) -> Self {
        Self {
            target: signed_target.unsigned_abs(),
            trim_larger,
            allow_fractions,
            bounds: Some(bounds),
        }
    }

    pub fn unbounded(signed_target: Ratio<i64>, trim_larger: bool, allow_fractions: bool) -> Self {
        Self {
            target: signed_target.unsigned_abs(),
            trim_larger,
            allow_fractions,
            bounds: None,
        }
    }

    fn test_value(&self, new_value: Ratio<u64>) -> HexResult<Ratio<u64>> {
        if self.trim_larger && new_value > self.target
            || !self.allow_fractions && !new_value.is_integer()
        {
            return Err(HexError::OutOfLimits);
        }
        Ok(new_value)
    }

    fn test_bounds(&self, new_bounds: impl Into<Bounds>) -> HexResult<Bounds> {
        let new_bounds = new_bounds.into();
        match self.bounds {
            Some(bounds) if !new_bounds.fits_in(bounds) => Err(HexError::OutOfLimits),
            _ => Ok(new_bounds),
        }
    }
}

#[derive(Clone)]
pub struct Path {
    value: Ratio<u64>,
    segments: Vec<Segment>,
    segments_set: HashSet<Segment>,
    points_set: HashSet<Coord>,
    minmax: MinMax,
}

impl Path {
    pub fn zero(sign: NonZeroSign) -> Self {
        let segments = match sign {
            NonZeroSign::Positive => get_pattern_segments(Direction::SouthEast, "aqaa"),
            NonZeroSign::Negative => get_pattern_segments(Direction::NorthEast, "dedd"),
        }
        .unwrap();

        Self {
            value: 0.into(),
            segments: segments.clone(),
            segments_set: HashSet::from_iter(segments.clone()),
            points_set: HashSet::from_iter(
                segments
                    .iter()
                    .flat_map(|segment| [segment.root(), segment.end()]),
            ),
            minmax: MinMax::from(&segments),
        }
    }

    pub fn value(&self) -> Ratio<u64> {
        self.value
    }

    pub fn bounds(&self) -> Bounds {
        self.minmax.into()
    }

    pub fn len(&self) -> usize {
        self.segments.len()
    }

    pub fn num_points(&self) -> usize {
        self.points_set.len()
    }

    pub fn next_segment(&self, angle: Angle) -> HexResult<Segment> {
        let new_segment = self.segments.last().unwrap().next_segment(angle);
        if self.segments_set.contains(&new_segment) {
            return Err(HexError::SegmentAlreadyExists(new_segment));
        }
        Ok(new_segment)
    }

    pub fn should_replace(&self, other: &Option<Path>) -> bool {
        match other {
            Some(other) => self.bounds().is_better_than(other.bounds()),
            None => true,
        }
    }

    pub fn try_with_angle<F>(&self, angle: Angle, limits: PathLimits, f: F) -> HexResult<Self>
    where
        F: FnOnce(&Path) -> bool,
    {
        // find the value of the new path and check if it's within limits
        let new_value = angle.apply_to(self.value)?;
        limits.test_value(new_value)?;

        // find the segment being added and check if it's within limits and doesn't overlap
        let new_segment = self.next_segment(angle)?;
        let new_point = new_segment.end();
        let new_minmax = self.minmax.with_point(new_point);
        limits.test_bounds(new_minmax)?;

        // construct the new path after the above checks to save effort in the negative case
        let new_path = Self {
            value: new_value,
            minmax: new_minmax,
            segments: cloned_push(&self.segments, new_segment),
            segments_set: cloned_union_single(&self.segments_set, new_segment),
            points_set: cloned_union_single(&self.points_set, new_point),
        };

        // finally, check the new path against the current smallest path (want acquiring the lock to be done as little as possible)
        // with the current should_replace() impl, this could be done before construction, but that may change in the future
        match f(&new_path) {
            true => Ok(new_path),
            false => Err(HexError::OutOfLimits),
        }
    }

    pub fn starting_direction(&self) -> Direction {
        self.segments[0].direction()
    }

    pub fn pattern(&self) -> String {
        self.segments
            .iter()
            .tuple_windows()
            .map(|(a, b)| char::from(b.direction().angle_from(a.direction())))
            .collect()
    }
}
