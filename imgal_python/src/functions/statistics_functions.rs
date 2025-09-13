use numpy::{PyReadwriteArray1, ndarray::Array1};
use pyo3::prelude::*;

use crate::error::map_array_error;
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

/// Sort 1-dimensional arrays of values and their associated weights.
///
/// This function performs a bottom up merge sort on the input 1-dimensional
/// data array along with it's associated weights. Both the "data" and "weights"
/// arrays are mutated during the sorting. The output of this function is a
/// weighted inversion count.
///
/// :param data: A 1-dimensional array/slice of numbers of the same length as
///    "weights".
/// :param weights: A 1-dimensional array/slice of weights of the same length as
///    "data".
/// :return: The number of swaps needed to sort the input array.
#[pyfunction]
#[pyo3(name = "weighted_merge_sort_mut")]
pub fn statistics_weighted_merge_sort_mut<'py>(
    data: Bound<'py, PyAny>,
    mut weights: PyReadwriteArray1<f64>,
) -> PyResult<f64> {
    // pattern match and extract the allowed array type
    if let Ok(mut d) = data.extract::<PyReadwriteArray1<f64>>() {
        return statistics::weighted_merge_sort_mut(
            d.as_slice_mut().unwrap(),
            weights.as_slice_mut().unwrap(),
        )
        .map(|output| output)
        .map_err(map_array_error);
    } else if let Ok(mut d) = data.extract::<PyReadwriteArray1<i32>>() {
        return statistics::weighted_merge_sort_mut(
            d.as_slice_mut().unwrap(),
            weights.as_slice_mut().unwrap(),
        )
        .map(|output| output)
        .map_err(map_array_error);
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype.",
        ));
    }
}
