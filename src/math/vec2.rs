use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<N>
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
    pub x: N,
    pub y: N,
}

impl<N> Add for Vec2<N>
where
    N: Add<Output = N>
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
    type Output = Vec2<N>;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N> AddAssign for Vec2<N>
where
    N: Add<Output = N>
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
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

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
