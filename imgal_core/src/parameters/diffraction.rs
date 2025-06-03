/// Compute the Abbe diffraction limit.
///
/// # Description
///
/// This function computes Ernst Abbe's diffraction limit
/// for a microscope using:
///
/// d = wavelength / 2 * NA
///
/// Where NA is the numerical aperture of the objective.
///
/// # Arguments
///
/// * `wavelength`: The wavelength of light in nanometers.
/// * `na`: The numerical aperture.
///
/// # Returns
///
/// * `f64`: Abbe's diffraction limit.
pub fn abbe_diffraction_limit(wavelength: f64, na: f64) -> f64 {
    wavelength / (2.0 * na)
}
