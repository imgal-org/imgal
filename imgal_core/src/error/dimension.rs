use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionError {
    InvalidAxis { axis: usize, dim_len: usize },
}

impl fmt::Display for DimensionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DimensionError::InvalidAxis { axis, dim_len } => {
                write!(
                    f,
                    "Axis {} is out of bounds for dimension length {}.",
                    axis, dim_len
                )
            }
        }
    }
}

impl error::Error for DimensionError {}
