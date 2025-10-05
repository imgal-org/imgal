use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use imgal::threshold;

/// Create a boolean mask from a threshold value.
///
/// This function computes a threshold mask (as a boolean array) from the input
/// image at the given threshold value.
///
/// :param image: An n-dimensional image.
/// :param threshold: The image pixel threshold value.
/// :return: A boolean array of the same shape as the input image
///     with pixels that are greater than the threshold value set as "true"
///     and pixels that are below the threshold value set as "false".
#[pyfunction]
#[pyo3(name = "manual_mask")]
pub fn threshold_manual_mask<'py>(
    py: Python<'py>,
    image: Bound<'py, PyAny>,
    threshold: f64,
) -> PyResult<Bound<'py, PyArrayDyn<bool>>> {
    if let Ok(arr) = image.extract::<PyReadonlyArrayDyn<u8>>() {
        let output = threshold::manual_mask(arr.as_array(), threshold as u8);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = image.extract::<PyReadonlyArrayDyn<u16>>() {
        let output = threshold::manual_mask(arr.as_array(), threshold as u16);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = image.extract::<PyReadonlyArrayDyn<f32>>() {
        let output = threshold::manual_mask(arr.as_array(), threshold as f32);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = image.extract::<PyReadonlyArrayDyn<f64>>() {
        let output = threshold::manual_mask(arr.as_array(), threshold);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
}
