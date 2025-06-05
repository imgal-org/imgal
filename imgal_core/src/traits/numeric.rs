use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub};

/// These traits enable a lossy f64 to f32 conversion.
pub trait ConvertFromF64 {
    fn from_f64(x: f64) -> Self;
}

impl ConvertFromF64 for f32 {
    fn from_f64(x: f64) -> Self {
        x as f32
    }
}

impl ConvertFromF64 for f64 {
    fn from_f64(x: f64) -> Self {
        x
    }
}

pub trait FloatLike:
    Copy
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + ConvertFromF64
    + MulAssign
    + Into<f64>
    + Sync
{
}

impl<T> FloatLike for T where
    T: Copy
        + Add<Output = T>
        + Div<Output = T>
        + Mul<Output = T>
        + Sub<Output = T>
        + AddAssign
        + ConvertFromF64
        + MulAssign
        + Into<f64>
        + Sync
{
}
