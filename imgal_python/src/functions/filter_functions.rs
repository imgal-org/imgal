use numpy::{IntoPyArray, PyArray1, ndarray::Array1};
use pyo3::prelude::*;

use imgal_core::filter;

/// Convolve two 1-dimensional signals using the Fast Fourier Transform (FFT).
///
/// Compute the convolution of two discrete signals ("a" and "b") by transforming
/// them to the frequency domain, multiplying them, and then transforming the
/// result back into a signal.
///
/// :param a: The first input signal to FFT convolve. Typically the data signal
///     or the longest of the two signals.
/// :param b: The second input signal to FFT convolve. Typically the kernel or
///     instrument response function to convolve with.
/// :return: The FFT convolved result of the same length as the input signal "a".
#[pyfunction]
#[pyo3(name = "fft_convolve")]
pub fn filter_fft_convolve(
    py: Python,
    a: Vec<f64>,
    b: Vec<f64>,
) -> PyResult<Bound<PyArray1<f64>>> {
    let a_arr = Array1::from_vec(a);
    let b_arr = Array1::from_vec(b);
    let output = filter::fft_convolve(a_arr.view(), b_arr.view());
    Ok(output.into_pyarray(py))
}
