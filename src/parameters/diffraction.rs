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
pub fn abbe_diffraction_limit(wavelength: &i32, na: &f64) -> f64 {
    return (wavelength as f64 / (2 * na))
}
