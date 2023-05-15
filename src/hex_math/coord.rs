use std::ops::{Add, AddAssign, Neg, Sub};

use super::{Angle, Direction};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    q: i32,
    r: i32,
}

impl Coord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn q(&self) -> i32 {
        self.q
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn s(&self) -> i32 {
        -self.q - self.r
    }

    pub fn rotated(&self, angle: Angle) -> Self {
        let mut rotated = *self;
        for _ in 0..(angle as i32) {
            rotated = Self::new(-rotated.r, -rotated.s());
        }
        rotated
    }

    pub fn pixel(&self, size: f32) -> (f32, f32) {
        (
            size * (3_f32.sqrt() * (self.q as f32) + 3_f32.sqrt() / 2. * (self.r as f32)),
            size * (3. / 2. * (self.r as f32)),
        )
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.q + rhs.q, self.r + rhs.r)
    }
}

impl Add<Direction> for Coord {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<Direction> for Coord {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}

impl Neg for Coord {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.q, -self.r)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Sub<Direction> for Coord {
    type Output = Self;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + -rhs
    }
}

impl From<Direction> for Coord {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::NorthEast => Self::new(1, -1),
            Direction::East => Self::new(1, 0),
            Direction::SouthEast => Self::new(0, 1),
            Direction::SouthWest => Self::new(-1, 1),
            Direction::West => Self::new(-1, 0),
            Direction::NorthWest => Self::new(0, -1),
        }
    }
}
