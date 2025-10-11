use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use imgal::image;

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
