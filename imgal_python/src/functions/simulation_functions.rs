use numpy::{IntoPyArray, PyArray1, PyArray3, PyArrayMethods, PyReadonlyArray1};
use pyo3::prelude::*;

use imgal_core::simulation;

/// Simulate a 1-dimensional gaussian IRF convolved decay curve.
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (1-dimensional) by FFT convolving the IRF with a decay cruve. The ideal
/// decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// The ideal decay curve is then convolved with a Guassian IRF.
///
/// :param samples: The number of descrete points that make up the decay curve (i.e. time).
/// :param period: The period, in the same unit as thee other parameters(e.g. seconds).
/// :param tau: The lifetime, in the same unit as the other parameters (e.g. seconds).
/// :param initial_value: The initial fluorescence value.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :return: The 1-dimensional Gaussian IRF convolved decay curve.
#[pyfunction]
#[pyo3(name = "gaussian_fluorescence_1d")]
pub fn decay_gaussian_fluorescence_1d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::decay::gaussian_fluorescence_1d(
        samples,
        period,
        tau,
        initial_value,
        irf_width,
        irf_center,
    );
    Ok(output.into_pyarray(py))
}

/// Simulate a 3-dimensional Gaussian IRF convolved decay curve.
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (3-dimensional) by FFT convolving the IRF with a decay cruve. The ideal
/// decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// The ideal decay curve is then convolved with a Guassian IRF.
///
/// :param samples: The number of descrete points that make up the decay curve (i.e. time).
/// :param period: The period, in the same unit as thee other parameters(e.g. seconds).
/// :param tau: The lifetime, in the same unit as the other parameters (e.g. seconds).
/// :param initial_value: The initial fluorescence value.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :param shape: The row and col shape to broadcast the decay curve into.
///
/// # Returns
///
/// * `Array3<f64>`: The 3-dimensional Gaussian IRF convolved decay curve.
#[pyfunction]
#[pyo3(name = "gaussian_fluorescence_3d")]
pub fn decay_gaussian_fluorescence_3d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    let output = simulation::decay::gaussian_fluorescence_3d(
        samples,
        period,
        tau,
        initial_value,
        irf_width,
        irf_center,
        shape,
    );
    Ok(output.into_pyarray(py))
}

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
/// :param period: The period in the same unit as tau (e.g. seconds).
/// :param tau: The lifetime in the same unit as the period (e.g. seconds).
/// :param initial_value: The initial fluorescence value.
/// :return: The 1-dimensional decay curve.
#[pyfunction]
#[pyo3(name = "ideal_fluorescence_1d")]
pub fn decay_ideal_fluorescence_1d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::decay::ideal_fluorescence_1d(samples, period, tau, initial_value);
    Ok(output.into_pyarray(py))
}

/// Simulate a 3-dimensional fluorescence decay curve.
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// Where "Io" is the initial fluorescence value and "t" is the time (i.e. the
/// number of samples). The decay curve is then broadcasted to the specified input
/// shape with the number of samples along the last axis.
///
/// :param samples: The number of descrete points that make up the decay curve
///     (i.e. time).
/// :param period: The period in the same unit as tau (e.g. seconds).
/// :param tau: The lifetime in the same unit as the period (e.g. seconds).
/// :param initial_value: The initial fluorescence value.
/// :param shape: The row and col shape to broadcast the decay curve into.
/// :return: The 3-dimensional decay curve.
#[pyfunction]
#[pyo3(name = "ideal_fluorescence_3d")]
pub fn decay_ideal_fluorescence_3d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> PyResult<Bound<PyArray3<f64>>> {
    let output =
        simulation::decay::ideal_fluorescence_3d(samples, period, tau, initial_value, shape);
    Ok(output.into_pyarray(py))
}

/// Simulate a 1-dimensional Gaussian instruement response function (IRF).
///
/// This function creates a Gaussian IRF by converting "full width at half maximum"
/// (FWHM) parameters into a normalized Gaussian distribution. The FWHM is
/// converted to standard deviation using the relationship:
///
/// ```text
/// σ = FWHM / (2 × √(2 × ln(2)))
/// ```
/// where `ln(2) ≈ 0.693147` is the natural logarithm of 2.
///
/// :param bins: The number of discrete points to sample the Gaussian distribution.
/// :param time_range: The total time range over which to simulate the IRF.
/// :param irf_width: The full width at half maximum (FWHM) of the IRF.
/// :param irf_center: The temporal position of the IRF peak within the time range.
/// :return : The simulated 1-dimensional IRF curve.
#[pyfunction]
#[pyo3(name = "gaussian_irf_1d")]
pub fn instrument_gaussian_irf_1d(
    py: Python,
    bins: usize,
    time_range: f64,
    irf_width: f64,
    irf_center: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::instrument::gaussian_irf_1d(bins, time_range, irf_width, irf_center);
    Ok(output.into_pyarray(py))
}

/// Simulate Poisson noise on a 1-dimensional array.
///
/// Apply Poisson noise (i.e. shot noise) on a 1-dimensonal array of data. Here,
/// "data" typically represents signal data.
///
/// :param data: The input 1-dimensional array of signal values.
/// :param scale: The scale factor.
/// :param seed: The random number generator seed for reproducible results. If "None"
///     the default seed value is 0.
/// :return: The 1-dimensonal array with Poisson noise applied.
#[pyfunction]
#[pyo3(name = "noise_poisson")]
#[pyo3(signature = (data, scale, seed=None))]
pub fn poisson<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    scale: f64,
    seed: Option<u64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(array) = data.extract::<PyReadonlyArray1<f32>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array().to_owned();
        let output = simulation::noise::poisson(arr, scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray1<f64>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array().to_owned();
        let output = simulation::noise::poisson(arr, scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray1<u16>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array().to_owned();
        let output = simulation::noise::poisson(arr, scale, seed);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray1<u8>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array().to_owned();
        let output = simulation::noise::poisson(arr, scale, seed);
        return Ok(output.into_pyarray(py))
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype.",
        ));
    }
}
