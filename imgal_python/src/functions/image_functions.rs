use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use imgal::image;

/// Compute the image histogram from an n-dimensional array.
///
/// This function computes an image (_i.e._ frequency) histogram for the values
/// in the input n-dimensional array.
///
/// :param data: The input n-dimensional array to construct the histogram from.
/// :param bins: The number of bins to use for the histogram, default = 256.
/// :return: The histogram of the input n-dimensional array of size `bins`.
///     Each element represents the count of values falling into the
///     corresponding bin.
#[pyfunction]
#[pyo3(name = "histogram")]
#[pyo3(signature = (data, bins=None))]
pub fn image_histogram<'py>(data: Bound<'py, PyAny>, bins: Option<usize>) -> PyResult<Vec<i64>> {
    if let Ok(arr) = data.extract::<PyReadonlyArrayDyn<u8>>() {
        return Ok(image::histogram(arr.as_array(), bins));
    }
    if let Ok(arr) = data.extract::<PyReadonlyArrayDyn<u16>>() {
        return Ok(image::histogram(arr.as_array(), bins));
    }
    if let Ok(arr) = data.extract::<PyReadonlyArrayDyn<f32>>() {
        return Ok(image::histogram(arr.as_array(), bins));
    }
    if let Ok(arr) = data.extract::<PyReadonlyArrayDyn<f64>>() {
        return Ok(image::histogram(arr.as_array(), bins));
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
}
