use std::f64;

use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal_core::colocalization;

/// Compute colocalization strength using 2-dimensional Spatially Adaptive
/// Colocalization Analysis (SACA)
///
/// This function computes a pixel-wise _z-score_ indicating colocalization and
/// anti-colocalization strength on 2-dimensional input images using the
/// Spatially Adaptive Colocalization Analysis (SACA) framework. Per pixel SACA
/// utilizes a propagation and separation strategy to adaptively expand a
/// weighted circular kernel that defines the pixel of consideration's
/// neighborhood. The pixels within the neighborhood are assigned weights based
/// on their distance from the center pixel (decreasing with distance), ranked
/// and their colocalization coefficient computed using Kendall's Tau-b rank
/// correlation.
///
/// :param image_a: The 2-dimensional input image, `A`. Image `A` must have the same
///     shape as image `B`.
/// :param image_b: Ihe 2-dimensional input image, `B`. Image `B` must have the same
///     shape as image `A`.
/// :param threshold_a: Pixel intensity threshold value for image `A`. Pixels below
///     this value are given a weight of 0.0 if the pixel is in the circular
///     neighborhood.
/// :param threshold_b: Pixel intensity threshold value for image `B`. Pixels
///     below this value are given a weight of 0.0 if the pixel is in the
///     circular neighborhood.
/// :return: The pixel-wise _z-score_ indicating colocalization or
///     anti-colocalization by its sign and the degree or strength of the
///     relationship through its absolute values.
#[pyfunction]
#[pyo3(name = "saca_2d")]
pub fn colocalization_saca_2d<'py>(
    py: Python<'py>,
    image_a: PyReadonlyArray2<f64>,
    image_b: PyReadonlyArray2<f64>,
    threshold_a: f64,
    threshold_b: f64,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    colocalization::saca_2d(
        image_a.as_array(),
        image_b.as_array(),
        threshold_a,
        threshold_b,
    )
    .map(|output| output.into_pyarray(py))
    .map_err(map_array_error)
}
