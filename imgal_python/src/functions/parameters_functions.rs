use pyo3::prelude::*;

use imgal_core::parameters;

/// Python binding for parameters::abbe_diffraction_limit.
#[pyfunction]
#[pyo3(name = "abbe_diffraction_limit")]
pub fn parameters_abbe_diffraction_limit(wavelength: f64, na: f64) -> f64 {
    parameters::abbe_diffraction_limit(wavelength, na)
}

/// Python binding for parameters::omega.
#[pyfunction]
#[pyo3(name = "omega")]
pub fn parameters_omega(period: Bound<PyAny>) -> PyResult<f64> {
    let p: f64 = period.extract()?;
    Ok(parameters::omega(p))
}
