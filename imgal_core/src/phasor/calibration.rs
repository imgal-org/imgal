/// Calibrate an imaginary, S, phasor value.
///
/// # Description
///
/// Calibrate the imaginary, S, phasor value by rotating and scaling
/// by phase, ϕ, and modulation M respectively using:
///
/// g = M * cos(φ)\
/// s = M * sin(φ)\
/// S' = G * s + S * g
///
/// Where S' is the calibrated imaginary value after rotation and scaling.
///
/// # Arguments
///
/// * `g`: The real, G, component.
/// * `s`: The imaginary, S, component.
/// * `modulation`: The modulation of the phasor coordinate.
/// * `phi`: The phi, φ, polar angle of the phasor coordinate.
///
/// # Returns
///
/// * `f64` - The rotated and scaled (calibrated) imaginary, S, component.
pub fn imaginary(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    // compute the modulation and theta angle translations
    let g_trans = modulation * phi.cos();
    let s_trans = modulation * phi.sin();
    g * s_trans + s * g_trans
}

/// Calibrate a real, G, phasor value.
///
/// # Description
///
/// Calibrate the real, G, phasor value by rotating and scaling
/// by phase, ϕ, and modulation M respectively using:
///
/// g = M * cos(φ)\
/// s = M * sin(φ)\
/// G' = G * g - S * s
///
/// Where G' is the calibrated real value after rotation and scaling.
///
/// # Arguments
///
/// * `g`: The real, G, component.
/// * `s`: The imaginary, S, component.
/// * `modulation`: The modulation of the phasor coordinate.
/// * `phi`: The phi, φ, polar angle of the phasor coordinate.
///
/// # Returns
///
/// * `f64`: The rotated and scaled (calibrated) real, G, component.
pub fn real(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    // compute modulation and theta translations
    let g_trans = modulation * phi.cos();
    let s_trans = modulation * phi.sin();
    g * g_trans - s * s_trans
}
