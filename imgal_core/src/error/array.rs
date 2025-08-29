use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ArrayError {
    InvalidAxis { axis_idx: usize, dim_len: usize },
    InvalidSum { expected: f64, got: f64 },
    MismatchedArrayLengths { a_arr_len: usize, b_arr_len: usize },
}

// "Dimension size {} of axis {} is out of bounds for dimension size {}."
impl fmt::Display for ArrayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrayError::InvalidAxis { axis_idx, dim_len } => {
                write!(
                    f,
                    "Invalid axis, axis {} is out of bounds for dimension length {}.",
                    axis_idx, dim_len
                )
            }
            ArrayError::InvalidSum { expected, got } => {
                write!(f, "Invalid sum, expected {} but got {}.", expected, got)
            }
            ArrayError::MismatchedArrayLengths {
                a_arr_len,
                b_arr_len,
            } => {
                write!(
                    f,
                    "Mismatched array lengths, {} and {}, do not match.",
                    a_arr_len, b_arr_len
                )
            }
        }
    }
}

impl error::Error for ArrayError {}
