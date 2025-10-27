use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ImgalError {
    InvalidArrayGeneric {
        msg: &'static str,
    },
    InvalidArrayParameterValueEqual {
        param_name: &'static str,
        value: usize,
    },
    InvalidArrayParameterValueGreater {
        param_name: &'static str,
        value: usize,
    },
    InvalidArrayParameterValueLess {
        param_name: &'static str,
        value: usize,
    },
    InvalidAxis {
        axis_idx: usize,
        dim_len: usize,
    },
    InvalidSum {
        expected: f64,
        got: f64,
    },
    MismatchedArrayLengths {
        a_arr_len: usize,
        b_arr_len: usize,
    },
    MismatchedArrayShapes {
        shape_a: Vec<usize>,
        shape_b: Vec<usize>,
    },
}

// "Dimension size {} of axis {} is out of bounds for dimension size {}."
impl fmt::Display for ImgalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImgalError::InvalidArrayGeneric { msg } => {
                write!(f, "{}", msg)
            }
            ImgalError::InvalidArrayParameterValueEqual { param_name, value } => {
                write!(
                    f,
                    "Invalid array parameter value, the parameter {} can not equal {}.",
                    param_name, value
                )
            }
            ImgalError::InvalidArrayParameterValueGreater { param_name, value } => {
                write!(
                    f,
                    "Invalid array parameter value, the parameter {} can not be greater than {}.",
                    param_name, value
                )
            }
            ImgalError::InvalidArrayParameterValueLess { param_name, value } => {
                write!(
                    f,
                    "Invalid array parameter value, the parameter {} can not be less than {}.",
                    param_name, value
                )
            }
            ImgalError::InvalidAxis { axis_idx, dim_len } => {
                write!(
                    f,
                    "Invalid axis, axis {} is out of bounds for dimension length {}.",
                    axis_idx, dim_len
                )
            }
            ImgalError::InvalidSum { expected, got } => {
                write!(f, "Invalid sum, expected {} but got {}.", expected, got)
            }
            ImgalError::MismatchedArrayLengths {
                a_arr_len,
                b_arr_len,
            } => {
                write!(
                    f,
                    "Mismatched array lengths, {} and {}, do not match.",
                    a_arr_len, b_arr_len
                )
            }
            ImgalError::MismatchedArrayShapes { shape_a, shape_b } => {
                write!(
                    f,
                    "Mismatched array shapes, {:?} and {:?}, do not match.",
                    shape_a, shape_b
                )
            }
        }
    }
}

impl error::Error for ImgalError {}
