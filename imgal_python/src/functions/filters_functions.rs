use numpy::{IntoPyArray, ndarray::Array1, PyArray1};
use pyo3::prelude::*;

use imgal_core::filters;

/// FFT convolution filter
#[pyfunction]
#[pyo3(name = "fft_convolve")]
pub fn filters_fft_convolve(
    py: Python,
    a: Vec<f64>,
    b: Vec<f64>,
) -> PyResult<Bound<PyArray1<f64>>> {
    let a_arr = Array1::from_vec(a);
    let b_arr = Array1::from_vec(b);
    let output = filters::fft_convolve(a_arr.view(), b_arr.view());
    Ok(output.into_pyarray(py))
}
