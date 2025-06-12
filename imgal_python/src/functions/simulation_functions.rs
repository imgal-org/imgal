use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;

use imgal_core::simulation;

/// Python bindings for simulation::fluorescence_decay_1d
#[pyfunction]
#[pyo3(name = "fluorescence_decay_1d")]
pub fn decay_fluorescence_decay_1d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::decay::fluorescence_decay_1d(samples, period, tau, initial_value);
    Ok(output.into_pyarray(py))
}
