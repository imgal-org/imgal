use pyo3::PyErr;
use pyo3::exceptions::{PyException, PyIndexError, PyValueError};

use imgal_core::error::ArrayError;

/// Map ArrayError types to Python exceptions.
pub fn map_array_error(err: ArrayError) -> PyErr {
    match err {
        ArrayError::InvalidArrayGeneric { msg } => PyException::new_err(format!("{}", msg)),
        ArrayError::InvalidArrayParameterValueEqual { param_name, value } => {
            PyValueError::new_err(format!(
                "Invalid array parameter value, the parameter {} can not equal {}.",
                param_name, value
            ))
        }
        ArrayError::InvalidArrayParameterValueGreater { param_name, value } => {
            PyValueError::new_err(format!(
                "Invalid array parameter value, the parameter {} can not be greater than {}.",
                param_name, value
            ))
        }
        ArrayError::InvalidArrayParameterValueLess { param_name, value } => {
            PyValueError::new_err(format!(
                "Invalid array parameter value, the parameter {} can not be less than {}.",
                param_name, value
            ))
        }
        ArrayError::InvalidAxis { axis_idx, dim_len } => PyIndexError::new_err(format!(
            "Axis {} is out of bounds for dimension length {}.",
            axis_idx, dim_len
        )),
        ArrayError::InvalidSum { expected, got } => PyValueError::new_err(format!(
            "Invalid sum, expected {} but got {}.",
            expected, got
        )),
        ArrayError::MismatchedArrayLengths {
            a_arr_len,
            b_arr_len,
        } => PyValueError::new_err(format!(
            "Mismatched array lengths, {} and {}, do not match.",
            a_arr_len, b_arr_len
        )),
        ArrayError::MismatchedArrayShapes { shape_a, shape_b } => PyValueError::new_err(format!(
            "Mismatched array shapes, {:?} and {:?}, do not match.",
            shape_a, shape_b
        )),
    }
}
