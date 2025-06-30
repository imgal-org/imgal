use numpy::ndarray::Array1;
use pyo3::prelude::*;

use imgal_core::statistics;

/// Compute the sum of a sequence of numbers.
///
/// :param input: The sequence of numbers.
/// :return: The sum.
#[pyfunction]
#[pyo3(name = "sum")]
pub fn statistics_sum(input: Vec<f64>) -> f64 {
    let input_arr = Array1::from_vec(input);
    statistics::sum(input_arr.view())
}
