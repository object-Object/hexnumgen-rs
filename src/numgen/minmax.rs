use std::cmp::{max, min};

use super::Bounds;
use crate::hex_math::{Coord, Segment};

#[derive(Debug, Clone, Copy)]
pub struct MinMax {
    min_q: i32,
    max_q: i32,
    min_r: i32,
    max_r: i32,
    min_s: i32,
    max_s: i32,
}

impl MinMax {
    pub fn with_point(&self, point: Coord) -> Self {
        Self {
            min_q: min(self.min_q, point.q()),
            max_q: max(self.max_q, point.q()),
            min_r: min(self.min_r, point.r()),
            max_r: max(self.max_r, point.r()),
            min_s: min(self.min_s, point.s()),
            max_s: max(self.max_s, point.s()),
        }
    }
}

impl From<&Vec<Segment>> for MinMax {
    fn from(segments: &Vec<Segment>) -> Self {
        let mut min_q = segments[0].root().q();
        let mut max_q = segments[0].root().q();
        let mut min_r = segments[0].root().r();
        let mut max_r = segments[0].root().r();
        let mut min_s = segments[0].root().s();
        let mut max_s = segments[0].root().s();

        for segment in segments {
            for point in [segment.root(), segment.end()] {
                min_q = min(min_q, point.q());
                max_q = max(max_q, point.q());
                min_r = min(min_r, point.r());
                max_r = max(max_r, point.r());
                min_s = min(min_s, point.s());
                max_s = max(max_s, point.s());
            }
        }

        Self {
            min_q,
            max_q,
            min_r,
            max_r,
            min_s,
            max_s,
        }
    }
}

impl From<MinMax> for Bounds {
    fn from(minmax: MinMax) -> Bounds {
        Bounds::new(
            (minmax.max_q - minmax.min_q + 1) as u32,
            (minmax.max_r - minmax.min_r + 1) as u32,
            (minmax.max_s - minmax.min_s + 1) as u32,
        )
    }
}
