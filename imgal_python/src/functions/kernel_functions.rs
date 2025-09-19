use numpy::{IntoPyArray, PyArray2, PyArray3};
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal_core::kernel;

/// Create a 2-dimensional square kernel with a circle neighborhood.
///
/// This function creates a square boolean kernel representing a filled circle
/// of the specified radius (i.e. the neighborhood). The circle is defined
/// using the Euclidean distance from the center point. Points within the
/// radius are set to "true", while points outside are set to "false".
///
/// :param radius: The radius of the circle in pixels. Must be greather than 0.
/// :return: A 2-dimensional square boolean array with side lengths
///     of "radius * 2 + 1" where "true" values represent points inside or on
///     the circle boundary of the specified radius.
#[pyfunction]
#[pyo3(name = "circle")]
pub fn neighborhood_circle(py: Python, radius: usize) -> PyResult<Bound<PyArray2<bool>>> {
    kernel::neighborhood::circle(radius)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}

/// Create a 3-dimensional cube kernel with a sphere neighborhood.
///
/// This function creates a cube boolean kernel representing a filled sphere of
/// the specified radius (i.e the neighborhood). The sphere is defined using
/// the Euclidean distance from the center point. Points within the radius are
/// set to "true", while jpoints outside are set to "false".
///
/// :param radius: The radius of the sphere in voxels. Must be greater than 0.
/// :return: A 3-dimensional cube boolean array with side lengths
///     of "radius * 2 + 1" where "true" values represent points inside or on
///     the sphere boundary of the specified radius.
#[pyfunction]
#[pyo3(name = "sphere")]
pub fn neighborhood_sphere(py: Python, radius: usize) -> PyResult<Bound<PyArray3<bool>>> {
    kernel::neighborhood::sphere(radius)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}
