use super::Vec2;

use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aabb<N>
where
    N: Add<Output = N>
        + AddAssign
        + Clone
        + Copy
        + Div<Output = N>
        + DivAssign
        + Eq
        + Mul<Output = N>
        + MulAssign
        + Ord
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Sub<Output = N>
        + SubAssign,
{
    pub min: Vec2<N>,
    pub max: Vec2<N>,
}

impl<N> Aabb<N>
where
    N: Add<Output = N>
        + AddAssign
        + Clone
        + Copy
        + Div<Output = N>
        + DivAssign
        + Eq
        + Mul<Output = N>
        + MulAssign
        + Ord
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Sub<Output = N>
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

impl<N> Add<Vec2<N>> for Aabb<N>
where
    N: Add<Output = N>
        + AddAssign
        + Clone
        + Copy
        + Div<Output = N>
        + DivAssign
        + Eq
        + Mul<Output = N>
        + MulAssign
        + Ord
        + PartialEq
        + PartialOrd
        + std::fmt::Debug
        + Sub<Output = N>
        + SubAssign,
{
    type Output = Self;

    fn add(self, rhs: Vec2<N>) -> Self::Output {
        Self {
            min: self.min + rhs,
            max: self.max + rhs,
        }
    }
}
