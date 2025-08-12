use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionError {
    InvalidAxis {
        axis_idx: usize,
        dim_len: usize,
    },
    InvalidDimensionSize {
        dim_a: usize,
        dim_b: usize,
        axis_idx: usize,
    },
}
// "Dimension size {} of axis {} is out of bounds for dimension size {}."
impl fmt::Display for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DimensionError::InvalidAxis { axis_idx, dim_len } => {
                write!(
                    f,
                    "Axis {} is out of bounds for dimension length {}.",
                    axis_idx, dim_len
                )
            }
            DimensionError::InvalidDimensionSize {
                dim_a,
                dim_b,
                axis_idx,
            } => {
                write!(
                    f,
                    "Dimension size {} of axis {} is out of bounds for dimension size {}.",
                    dim_a, axis_idx, dim_b
                )
            }
        }
    }
}

impl error::Error for DimensionError {}
