use itertools::Itertools;
use std::collections::HashSet;

use crate::{
    errors::HexError,
    hex_math::{get_pattern_segments, Angle, Coord, Direction, Segment},
    utils::{cloned_push, cloned_union_single, NonZeroSign},
};

use super::{Bounds, MinMax};

#[derive(Clone)]
pub struct Path {
    sign: NonZeroSign,
    value: i32,
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
            sign,
            value: 0,
            segments: segments.clone(),
            segments_set: HashSet::from_iter(segments.clone()),
            points_set: HashSet::from_iter(segments.iter().flat_map(|segment| [segment.root(), segment.end()])),
            minmax: MinMax::from(&segments),
        }
    }

    pub fn value(&self) -> i32 {
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

    pub fn with_angle(&self, angle: Angle) -> Result<Self, HexError> {
        let new_value = angle.apply_to_int(self.value)?;
        let new_segment = self.segments.last().unwrap().next_segment(angle);
        let new_point = new_segment.end();

        if self.segments_set.contains(&new_segment) {
            return Err(HexError::SegmentAlreadyExists(new_segment));
        }

        Ok(Self {
            sign: self.sign,
            value: new_value,
            segments: cloned_push(&self.segments, new_segment),
            segments_set: cloned_union_single(&self.segments_set, new_segment),
            points_set: cloned_union_single(&self.points_set, new_point),
            minmax: self.minmax.with_point(new_point),
        })
    }

    pub fn starting_direction(&self) -> Direction {
        self.segments[0].direction()
    }

    pub fn pattern(&self) -> String {
        self.segments.iter().tuple_windows().map(|(a, b)| char::from(b.direction().angle_from(a.direction()))).collect()
    }
}
