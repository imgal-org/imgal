use std::f64;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal_core::colocalization;

#[pyfunction]
#[pyo3(name = "saca_2d")]
pub fn colocalization_saca_2d<'py>(
    py: Python<'py>,
    data_a: PyReadonlyArray2<f64>,
    data_b: PyReadonlyArray2<f64>,
    threshold_a: f64,
    threshold_b: f64,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    colocalization::saca_2d(
        data_a.as_array(),
        data_b.as_array(),
        threshold_a,
        threshold_b,
    )
    .map(|output| output.into_pyarray(py))
    .map_err(map_array_error)
}
