use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

pub trait ToFloat64:
    Copy
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + MulAssign
    + Sum
    + Debug
    + Default
    + Into<f64>
    + PartialOrd
    + Send
    + Sync
{
}

impl<T> ToFloat64 for T where
    T: Copy
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign
        + MulAssign
        + Sum
        + Debug
        + Default
        + Into<f64>
        + PartialOrd
        + Send
        + Sync
{
}
