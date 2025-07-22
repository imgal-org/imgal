use ndarray::{ArrayView3, ArrayViewMut3, Axis};
use rayon::prelude::*;

use crate::phasor::plot;

/// Calibrate a real and imaginary (G, S) coordinate pair.
///
/// # Description
///
/// Calibrate the real and imaginary (_e.g._ G and S) coordinate pair by rotating
/// and scaling by phase (φ) and modulation (M) respectively using:
///
/// ```text
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
/// ```
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// # Arguments
///
/// * `g`: The real component (G) to calibrate.
/// * `s`: The imaginary (S) to calibrate.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phase`: The phase, φ angle, to rotate the input (G, S) coordinates.
///
/// # Returns
///
/// * `(f64, f64)`: The calibrated coordinate pair, (G, S).
pub fn coordinate_pair(g: f64, s: f64, modulation: f64, phase: f64) -> (f64, f64) {
    let g_trans = modulation * phase.cos();
    let s_trans = modulation * phase.sin();
    let g_cal = g * g_trans - s * s_trans;
    let s_cal = g * s_trans + s * g_trans;
    (g_cal, s_cal)
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensonal phasor
/// image.
///
/// # Description
///
/// This function calibrates an input 3-dimensonal phasor image by rotating and
/// scaling G and S coordinates by phase (φ) and modulation (M) respectively using:
///
/// ```text
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
/// ```
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling. This function mutates the input data and does not create a new
/// array.
///
/// # Arguments
///
/// * `data`: The 3-dimensonal phasor image, where G and S are channels 0 and 1
///    respectively.
/// * `modulation`: The modulation to scale the input (G, S) coordinates.
/// * `phase`: The phase, φ angle, to rotate the input (G, S) coordinates.
/// * `axis`: The channel axis, default = 2.
pub fn image_mut(mut data: ArrayViewMut3<f64>, modulation: f64, phase: f64, axis: Option<usize>) {
    // set optional axis parameter if needed
    let a = axis.unwrap_or(2);

    // initialize calibration parameters
    let g_trans = modulation * phase.cos();
    let s_trans = modulation * phase.sin();

    let lanes = data.lanes_mut(Axis(a));
    lanes.into_iter().par_bridge().for_each(|mut ln| {
        let g_cal = ln[0] * g_trans - ln[1] * s_trans;
        let s_cal = ln[0] * s_trans + ln[1] * g_trans;
        ln[0] = g_cal;
        ln[1] = s_cal;
    });
}

/// Find the modulation and phase calibration values.
///
/// # Description
///
/// This function calculates the modulation and phase calibration values from
/// a pair of theoretical single component coordinates and the center of mass
/// coordinates of the measured real data.
///
/// # Arguments
///
/// * `data`: The 3-dimensional phasor image, where G and S are channels 0 and 1
///  respectively.
/// * `tau`: The lifetime, τ.
/// * `omega`: The angular frequency, ω.
/// * `axis`: The channel axis, default = 2.
///
/// # Returns
///
/// * `(f64, f64)`: The modulation and phase calibration values, (M, φ).
pub fn modulation_and_phase(
    data: &ArrayView3<f64>,
    tau: f64,
    omega: f64,
    axis: Option<usize>,
) -> (f64, f64) {
    // set optional axis parameter if needed
    let a = axis.unwrap_or(2);

    // get calibration modulation and phase
    let cal_point = plot::single_component_coordinate_pair(tau, omega);
    let cal_mod = plot::modulation(cal_point.0, cal_point.1);
    let cal_phs = plot::phase(cal_point.0, cal_point.1);

    // get data modulation and phase
    let data_center_g = data.index_axis(Axis(a), 0).mean().unwrap_or(0.0);
    let data_center_s = data.index_axis(Axis(a), 1).mean().unwrap_or(0.0);
    let data_mod = plot::modulation(data_center_g, data_center_s);
    let data_phs = plot::phase(data_center_g, data_center_s);

    // find delta values
    let d_mod = cal_mod / data_mod;
    let d_phs = cal_phs - data_phs;

    (d_mod, d_phs)
}
