use numpy::{IntoPyArray, PyArray3, PyArrayMethods, PyReadonlyArray3};
use pyo3::prelude::*;

use imgal_core::phasor;

/// Compute the real and imaginary (G, S) coordinates of a 3-dimensional decay
/// image.
///
/// The real (G) and imaginary (S) components are calculated using the normalized
/// sine and cosine Fourier transforms:
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// :param i_data: I(t), the decay data image.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :param axis: The decay or lifetime axis, default = 2.
/// :return: The real and imaginary coordinates as a 3-dimensional (row, col, ch)
///     image, where G and S are indexed at 0 and 1 respectively on the channel axis.
#[pyfunction]
#[pyo3(name = "image")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None, axis=None))]
pub fn time_domain_image<'py>(
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

/// Compute the imaginary (S) component of a 1-dimensional decay curve.
///
/// The imaginary (S) component is calculated using the normalized sine Fourier
/// transform:
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// :param i_data: I(t), the 1-dimensional decay curve.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :return: The imaginary component, S.
#[pyfunction]
#[pyo3(name = "imaginary")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None))]
pub fn time_domain_imaginary(
    i_data: Vec<f64>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    phasor::time_domain::imaginary(&i_data, period, harmonic, omega)
}

/// Compute the real (G) component of a 1-dimensional decay curve.
///
/// The real (G) component is calculated using the normalized cosine Fourier
/// transform:
///
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// :param i_data: I(t), the 1-dimensional decay curve.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :return: The real component, G.
#[pyfunction]
#[pyo3(name = "real")]
#[pyo3(signature = (i_data, period, harmonic=None, omega=None))]
pub fn time_domain_real(
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
pub fn calibration_imaginary(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    phasor::calibration::imaginary(g, s, modulation, phi)
}

/// Python binding for phasor::calibration::real.
#[pyfunction]
#[pyo3(name = "real")]
pub fn calibration_real(g: f64, s: f64, modulation: f64, phi: f64) -> f64 {
    phasor::calibration::real(g, s, modulation, phi)
}

/// Python binding for phasor::plot::multi_component_modulation.
#[pyfunction]
#[pyo3(name = "multi_component_modulation")]
pub fn plot_multi_component_modulation(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_modulation(g, s)
}

/// Python binding for phasor::plot::multi_component_phi.
#[pyfunction]
#[pyo3(name = "multi_component_phi")]
pub fn plot_multi_component_phi(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_phi(g, s)
}

/// Python binding for phasor::plot::single_component_modulation.
#[pyfunction]
#[pyo3(name = "single_component_modulation")]
pub fn plot_single_component_modulation(phi: f64) -> f64 {
    phasor::plot::single_component_modulation(phi)
}

/// Python binding for phasor::plot::single_component_phi.
#[pyfunction]
#[pyo3(name = "single_component_phi")]
pub fn plot_single_component_phi(omega: f64, tau: f64) -> f64 {
    phasor::plot::single_component_phi(omega, tau)
}
