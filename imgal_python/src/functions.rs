use numpy::{IntoPyArray, PyArray1, PyArray3, PyArrayMethods, PyReadonlyArray3};
use pyo3::prelude::*;

use imgal_core::integrate;
use imgal_core::parameters;
use imgal_core::phasor;
use imgal_core::simulation;
use imgal_core::statistics;

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
pub fn py_fn_parameters_abbe_diffraction_limit(wavelength: f64, na: f64) -> f64 {
    parameters::abbe_diffraction_limit(wavelength, na)
}

/// Python binding for parameters::omega.
#[pyfunction]
#[pyo3(name = "omega")]
pub fn py_fn_parameters_omega(period: Bound<PyAny>) -> PyResult<f64> {
    let p: f64 = period.extract()?;
    Ok(parameters::omega(p))
}

/// Python binding for phasor::time_domain::image.
#[pyfunction]
#[pyo3(name = "image")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None, axis=None))]
pub fn py_fn_phasor_time_domain_image<'py>(
    py: Python<'py>,
    i_data: Bound<'py, PyAny>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    // try and extract allowed array types
    if let Ok(array) = i_data.extract::<PyReadonlyArray3<f32>>() {
        let ro_array = array.readonly();
        let data = ro_array.as_array();
        let output = phasor::time_domain::image(&data, period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = i_data.extract::<PyReadonlyArray3<f64>>() {
        let ro_array = array.readonly();
        let data = ro_array.as_array();
        let output = phasor::time_domain::image(&data, period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = i_data.extract::<PyReadonlyArray3<u16>>() {
        let ro_array = array.readonly();
        let data = ro_array.as_array();
        let output = phasor::time_domain::image(&data, period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsopported array dtype.",
        ));
    };
}

/// Python binding for phasor::time_domain::imaginary.
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

/// Python binding for phasor::time_domain::real.
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

/// Python binding for phasor::calibration::imaginary.
#[pyfunction]
#[pyo3(name = "imaginary")]
pub fn py_fn_phasor_calibration_imaginary(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    phasor::calibration::imaginary(g, s, modulation, phi)
}

/// Python binding for phasor::calibration::real.
#[pyfunction]
#[pyo3(name = "real")]
pub fn py_fn_phasor_calibration_real(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    phasor::calibration::real(g, s, modulation, phi)
}

/// Python binding for phasor::plot::multi_component_modulation.
#[pyfunction]
#[pyo3(name = "multi_component_modulation")]
pub fn py_fn_phasor_plot_multi_component_modulation(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_modulation(g, s)
}

/// Python binding for phasor::plot::multi_component_phi.
#[pyfunction]
#[pyo3(name = "multi_component_phi")]
pub fn py_fn_phasor_plot_multi_component_phi(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_phi(g, s)
}

/// Python binding for phasor::plot::single_component_modulation.
#[pyfunction]
#[pyo3(name = "single_component_modulation")]
pub fn py_fn_phasor_plot_single_component_modulation(phi: f64) -> f64 {
    phasor::plot::single_component_modulation(phi)
}

/// Python binding for phasor::plot::single_component_phi.
#[pyfunction]
#[pyo3(name = "single_component_phi")]
pub fn py_fn_phasor_plot_single_component_phi(omega: f64, tau: f64) -> f64 {
    phasor::plot::single_component_phi(omega, tau)
}

/// Python bindings for simulation::fluorescence_decay_1d
#[pyfunction]
#[pyo3(name = "fluorescence_decay_1d")]
pub fn py_fun_simulation_decay_fluorescence_decay_1d(
    py: Python,
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> PyResult<Bound<PyArray1<f64>>> {
    let output = simulation::decay::fluorescence_decay_1d(samples, period, tau, initial_value);
    Ok(output.into_pyarray(py))
}

/// Python binding for statistics::sum
#[pyfunction]
#[pyo3(name = "sum")]
pub fn py_fn_statistics_sum(input: Vec<f64>) -> f64 {
    statistics::sum(&input)
}
