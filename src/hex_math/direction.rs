use std::fmt::Display;

use super::Angle;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    NorthEast = 0,
    East = 1,
    SouthEast = 2,
    SouthWest = 3,
    West = 4,
    NorthWest = 5,
}

impl Direction {
    pub fn is_east(&self) -> bool {
        matches!(*self, Self::NorthEast | Self::East | Self::SouthEast)
    }

    pub fn angle_from(&self, other: Self) -> Angle {
        Angle::from(*self as i32 - other as i32)
    }

    pub fn rotated(&self, angle: Angle) -> Self {
        Direction::from(*self as i32 + angle as i32)
    }
}

impl From<i32> for Direction {
    fn from(num: i32) -> Self {
        match num.rem_euclid(6) {
            0 => Direction::NorthEast,
            1 => Direction::East,
            2 => Direction::SouthEast,
            3 => Direction::SouthWest,
            4 => Direction::West,
            5 => Direction::NorthWest,
            _ => unreachable!(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Direction::NorthEast => "NORTH_EAST",
            Direction::East => "EAST",
            Direction::SouthEast => "SOUTH_EAST",
            Direction::SouthWest => "SOUTH_WEST",
            Direction::West => "WEST",
            Direction::NorthWest => "NORTH_WEST",
        };
        write!(f, "{}", name)
    }
}
