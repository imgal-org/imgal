use pyo3::PyErr;
use pyo3::exceptions::{PyIndexError, PyValueError};

use imgal_core::error::DimensionError;

/// Map DimensionError types to Python exceptions.
pub fn map_dimension_error(err: DimensionError) -> PyErr {
    match err {
        DimensionError::InvalidAxis { axis_idx, dim_len } => PyIndexError::new_err(format!(
            "Axis {} is out of bounds for dimension length {}.",
            axis_idx, dim_len
        )),
        DimensionError::InvalidDimensionSize {
            dim_a,
            dim_b,
            axis_idx,
        } => PyValueError::new_err(format!(
            "Dimension size {} of axis {} is out of bounds for dimension size {}.",
            dim_a, dim_b, axis_idx
        )),
    }
}
