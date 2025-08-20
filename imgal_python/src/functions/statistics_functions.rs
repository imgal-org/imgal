use numpy::ndarray::Array1;
use pyo3::prelude::*;

use imgal_core::statistics;

/// Compute the sum of a sequence of numbers.
///
/// :param data: The sequence of numbers.
/// :return: The sum.
#[pyfunction]
#[pyo3(name = "sum")]
pub fn statistics_sum(data: Vec<f64>) -> f64 {
    let arr = Array1::from_vec(data);
    statistics::sum(arr.view())
}
