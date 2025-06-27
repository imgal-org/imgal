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
    let g_trans: f64 = modulation * phi.cos();
    let s_trans: f64 = modulation * phi.sin();
    let g_cal: f64 = g * g_trans - s * s_trans;
    let s_cal: f64 = g * s_trans + s * g_trans;
    (g_cal, s_cal)
}
