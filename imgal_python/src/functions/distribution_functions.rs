use numpy::{IntoPyArray, PyArray1};
use pyo3::prelude::*;

use imgal_core::distribution;

/// Generate a normalized Gaussian distribution over a specified range.
///
/// This function creates a discrete Gaussian distribution by sampling the continuous
/// Gaussian probability density function at evenly spaced points across a given range.
/// The resulting distribution is normalized so that all values sum to 1.0.i
/// The function implements the Gaussian probability density function:
///
/// f(x) = exp(-((x - μ)² / (2σ²)))
///
/// where:
/// - `x` is the position along the range
/// - `μ` is the center (mean)
/// - `σ` is the sigma (standard deviation)
///
///
/// :param sigma: The standard deviation of the Gaussian distribution (i.e. the width).
/// :param bins: The number of discrete points to sample the Gaussian distribution.
/// :param range: The total width of the sampling range.
/// :param center: The mean (center) of the Gaussian distribution (i.e. the peak).
/// :return: The normalized Gaussian distribution.
#[pyfunction]
#[pyo3(name = "gaussian")]
pub fn distribution_gaussian(
    py: Python,
    sigma: f64,
    bins: usize,
    range: f64,
    center: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = distribution::gaussian(sigma, bins, range, center);
    Ok(output.into_pyarray(py))
}
