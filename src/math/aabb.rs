use super::Vec2;

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub struct Aabb<N>
where
    N: Add
        + AddAssign
        + Clone
        + Copy
        + Div
        + DivAssign
        + Eq
        + Mul
        + MulAssign
        + Ord
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Sub
        + SubAssign,
{
    pub min: Vec2<N>,
    pub max: Vec2<N>,
}

impl<N> Aabb<N>
where
    N: Add
        + AddAssign
        + Clone
        + Copy
        + Div
        + DivAssign
        + Eq
        + Mul
        + MulAssign
        + Ord
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Sub
        + SubAssign,
{
    /// Whether the AABB contains the given point
    pub fn contains(&self, point: Vec2<N>) -> bool {
        self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
    }
}
