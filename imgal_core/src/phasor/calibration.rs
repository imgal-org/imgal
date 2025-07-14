use ndarray::{Axis, Array3};
use rayon::prelude::*;

/// Calibrate a real and imaginary (G, S) coordinate pair.
///
/// # Description
///
/// Calibrate the real and imaginary (_e.g._ G and S) coordinate pair by rotating
/// and scaling by phase (φ) and modulation (M) respectively using:
///
/// g = M * cos(φ)\
/// s = M * sin(φ)\
/// G' = G * g - S * s\
/// S' = G * s + S * g
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// # Arguments
///
/// * `g`: The real component (G) to calibrate.
/// * `s`: The imaginary (S) to calibrate.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phi`: The phi, φ, polar angle to rotate the input (G, S) coordinates.
///
/// # Returns
///
/// * `f64`: The calibrated coordinate pair, (G, S).
pub fn coordinate_pair(g: f64, s: f64, modulation: f64, phi: f64) -> (f64, f64) {
    let g_trans = modulation * phi.cos();
    let s_trans = modulation * phi.sin();
    let g_cal = g * g_trans - s * s_trans;
    let s_cal = g * s_trans + s * g_trans;
    (g_cal, s_cal)
}

/// Calibrate an image
///
/// # Description
///
/// Mutates the input array.
pub fn image_mut(
    data: Array3<f64>,
    modulation: f64,
    phi: f64,
    axis: Option<usize>,
) {
    // set optional axis parameter if needed
    let a = axis.unwrap_or(2);

    // initialize calibration parameters
    let g_trans = modulation * phi.cos();
    let s_trans = modulation * phi.sin();

    // let g_arr = data.index_axis(Axis(a), 0);
    // let s_arr = data.index_axis(Axis(a), 1);
    let lanes = data.lanes(Axis(a));
    lanes.into_iter().par_bridge().for_each(|ln| {
        ln.iter().for_each(|x| {
        });
    });
}
