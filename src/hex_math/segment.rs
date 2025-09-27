use std::hash::Hash;

use super::{Angle, Coord, Direction};
use crate::errors::HexResult;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    root: Coord,
    direction: Direction,
}

impl Segment {
    pub fn new(root: Coord, direction: Direction) -> Self {
        Self { root, direction }
    }

    pub fn root(&self) -> Coord {
        self.root
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn end(&self) -> Coord {
        self.root + self.direction
    }

    pub fn rotated(&self, angle: Angle) -> Self {
        Self::new(self.root.rotated(angle), self.direction.rotated(angle))
    }

    pub fn next_segment(&self, angle: Angle) -> Self {
        Self::new(self.end(), self.direction.rotated(angle))
    }

    fn is_canonical(&self) -> bool {
        self.direction.is_east()
    }

    fn canonical_root(&self) -> Coord {
        if self.is_canonical() {
            self.root
        } else {
            self.root + self.direction
        }
    }

    fn canonical_direction(&self) -> Direction {
        if self.is_canonical() {
            self.direction
        } else {
            self.direction.rotated(Angle::Back)
        }
    }
}

impl Hash for Segment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.canonical_root().hash(state);
        self.canonical_direction().hash(state);
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.canonical_root() == other.canonical_root()
            && self.canonical_direction() == other.canonical_direction()
    }
}
impl Eq for Segment {}

pub fn get_pattern_segments(direction: Direction, pattern: &str) -> HexResult<Vec<Segment>> {
    let mut cursor = Coord::origin();
    let mut compass = direction;

    let mut segments = vec![Segment::new(cursor, compass)];

    for c in pattern.chars() {
        cursor += compass;
        compass = compass.rotated(Angle::try_from(c)?);

        segments.push(Segment::new(cursor, compass))
    }

    Ok(segments)
}
