use pyo3::prelude::*;

use imgal_core::filters;

/// FFT convolution filter
#[pyfunction]
#[pyo3(name = "fft_convolve")]
pub fn filters_fft_convolve(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    filters::fft_convolve(&a, &b)
}
