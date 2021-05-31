use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<N>
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
    pub x: N,
    pub y: N,
}

impl<N> Add for Vec2<N>
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
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<N> Sub for Vec2<N>
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
    type Output = Vec2<N>;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<N> SubAssign for Vec2<N>
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
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
