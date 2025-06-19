use pyo3::prelude::*;

use imgal_core::parameters;

/// Compute the Abbe diffraction limit.
///
/// Compute Ernst Abbe's diffraction limit using:
///
/// d = wavelength / 2 * NA
///
/// Where "NA" is the numerical aperture of the objective.
///
/// :param wavelength: The wavelength of light.
/// :param na: The numerical aperture.
/// :return: Abbe's diffraction limit.
#[pyfunction]
#[pyo3(name = "abbe_diffraction_limit")]
pub fn parameters_abbe_diffraction_limit(wavelength: f64, na: f64) -> f64 {
    parameters::abbe_diffraction_limit(wavelength, na)
}

/// Compute the angular frequency (omega) value.
///
/// Compute the angular frequency, omega (ω), using the following equation:
///
/// ω = 2π/T
///
/// Where "T" is the period.
///
/// :param period: The time period.
/// :return: The omega (ω) value.
#[pyfunction]
#[pyo3(name = "omega")]
pub fn parameters_omega(period: Bound<PyAny>) -> PyResult<f64> {
    let p: f64 = period.extract()?;
    Ok(parameters::omega(p))
}
