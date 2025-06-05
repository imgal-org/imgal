use std::ops::{Add, Div, Mul, Sub, AddAssign, MulAssign};

pub trait FloatLike:
    Copy
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + MulAssign
    + Into<f64>
    + From<f64>
    + Sync
{}

impl<T> FloatLike for T where
    T: Copy
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign
        + MulAssign
        + Into<f64>
        + From<f64>
        + Sync
{}
