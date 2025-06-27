use numpy::{IntoPyArray, PyArray1, PyArray3};
use pyo3::prelude::*;

use imgal_core::simulation;

/// Simulate a 1-dimensional fluorescence decay curve.
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// Where "Io" is the initial fluorescence value and "t" (i.e. the number of
/// samples).
///
/// :param samples: The number of descrete points that make up the decay curve
///     (i.e. time).
/// :param period: The period in the same unit as tau (e.g. nanoseconds).
/// :param tau: The lifetime in the same unit as the period (e.g. nanoseconds).
/// :param initial_value: The initial fluorescence value.
/// :return: The 1-dimensional decay curve.
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

/// Simulate a 3-dimensional fluorescence decay curve.
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// Where "Io" is the initial fluorescence value and "t" is the time (_i.e._ the)
/// number of samples). The decay curve is then broadcasted to the specified input
/// shape with the number of samples along the last axis.
///
/// :param samples: The number of descrete points that make up the decay curve
///     (i.e. time).
/// :param period: The period in the same unit as tau (e.g. nanoseconds).
/// :param tau: The lifetime in the same unit as the period (e.g. nanoseconds).
/// :param initial_value: The initial fluorescence value.
/// :param shape: The row and col shape to broadcast the decay curve into.
/// :return: The 3-dimensional decay curve.
#[pyfunction]
#[pyo3(name = "fluorescence_decay_3d")]
pub fn decay_fluorescence_decay_3d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    let output =
        simulation::decay::fluorescence_decay_3d(samples, period, tau, initial_value, shape);
    Ok(output.into_pyarray(py))
}
