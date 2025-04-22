/// Compute the Abbe diffraction limit.
///
/// This function computes Ernst Abbe's diffraction limit
/// for a microscope using:
///
/// d = wavelength / 2*NA
///
/// Where NA = numerical aperature
///
/// # Arguments
///
/// # Returns
///
pub fn abbe_diffraction_limit(wavelength: f64, na: f64) -> f64 {
    wavelength / (2.0 * na)
}
