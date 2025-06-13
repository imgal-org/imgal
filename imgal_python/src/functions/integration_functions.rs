use pyo3::prelude::*;

use imgal_core::integration;

/// Python binding for integrate::composite_simpson
#[pyfunction]
#[pyo3(name = "composite_simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn integration_composite_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integration::composite_simpson(&y, delta_x)
}

/// Python binding for integrate::midpoint.
#[pyfunction]
#[pyo3(name = "midpoint")]
#[pyo3(signature = (y, h=None))]
pub fn integration_midpoint(y: Vec<f64>, h: Option<f64>) -> f64 {
    integration::midpoint(&y, h)
}

/// Python binding for integrate::simpson.
#[pyfunction]
#[pyo3(name = "simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn integration_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integration::simpson(&y, delta_x).unwrap()
}
