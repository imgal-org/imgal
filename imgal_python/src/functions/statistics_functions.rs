use pyo3::prelude::*;

use imgal_core::statistics;

/// Python binding for statistics::sum
#[pyfunction]
#[pyo3(name = "sum")]
pub fn statistics_sum(input: Vec<f64>) -> f64 {
    statistics::sum(&input)
}
