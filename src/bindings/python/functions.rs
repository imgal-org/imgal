use pyo3::prelude::*;

use crate::integrate;
use crate::parameters;
use crate::phasor;
use crate::statistics;

/// Python binding for integrate::composite_simpson
#[pyfunction]
#[pyo3(name = "composite_simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn py_fn_integrate_composite_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integrate::composite_simpson(&y, delta_x)
}

/// Python binding for integrate::midpoint.
#[pyfunction]
#[pyo3(name = "midpoint")]
#[pyo3(signature = (y, h=None))]
pub fn py_fn_integrate_midpoint(y: Vec<f64>, h: Option<f64>) -> f64 {
    integrate::midpoint(&y, h)
}

/// Python binding for integrate::simpson.
#[pyfunction]
#[pyo3(name = "simpson")]
#[pyo3(signature = (y, delta_x=None))]
pub fn py_fn_integrate_simpson(y: Vec<f64>, delta_x: Option<f64>) -> f64 {
    integrate::simpson(&y, delta_x).unwrap()
}

/// Python binding for parameters::abbe_diffraction_limit.
#[pyfunction]
#[pyo3(name = "abbe_diffraction_limit")]
pub fn py_fn_parameters_abbe_diffraction_limit(wavelength: i32, na: f64) -> f64 {
    parameters::abbe_diffraction_limit(wavelength, na)
}

/// Python binding for parameters::omega.
#[pyfunction]
#[pyo3(name = "omega")]
pub fn py_fn_parameters_omega(period: f64) -> f64 {
    parameters::omega(period)
}

///Python binding for phasor::time_domain::imaginary.
#[pyfunction]
#[pyo3(name = "imaginary")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None))]
pub fn py_fn_phasor_time_domain_imaginary(
    i_data: Vec<f64>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    phasor::time_domain::imaginary(&i_data, period, harmonic, omega)
}

///Python binding for phasor::time_domain::real.
#[pyfunction]
#[pyo3(name = "real")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None))]
pub fn py_fn_phasor_time_domain_real(
    i_data: Vec<f64>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    phasor::time_domain::real(&i_data, period, harmonic, omega)
}

/// Python binding for statistics::sum
#[pyfunction]
#[pyo3(name = "sum")]
pub fn py_fn_statistics_sum(input: Vec<f64>) -> f64 {
    statistics::sum(&input)
}
