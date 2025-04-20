use pyo3::prelude::*;

use crate::integrate;
use crate::statistics;

/// Python binding for integrate::composite_simpson
#[pyfunction]
#[pyo3(name = "composite_simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn py_fn_integrate_composite_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integrate::composite_simpson(&y, delta_x)
}

#[pyfunction]
#[pyo3(name = "simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn py_fn_integrate_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integrate::simpson(&y, delta_x).unwrap()
}

#[pyfunction]
#[pyo3(name = "sum")]
pub fn py_fn_statistics_sum(input: Vec<f64>) -> f64 {
    statistics::sum(&input)
}
