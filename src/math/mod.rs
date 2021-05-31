use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait Number:
    Add
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
    + SubAssign
{
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<N>
where
    N: Number,
{
    pub x: N,
    pub y: N,
}

pub struct Aabb<N>
where
    N: Number,
{
    pub min: Vec2<N>,
    pub max: Vec2<N>,
}

impl<N> Aabb<N>
where
    N: Number,
{
    /// Whether the AABB contains the given point
    pub fn contains(&self, point: Vec2<N>) -> bool {
        self.min.x <= point.x
            && self.max.x >= point.x
            && self.min.y <= point.y
            && self.max.y >= point.y
    }
}
