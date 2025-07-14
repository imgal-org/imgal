use numpy::{
    IntoPyArray, PyArray3, PyArrayMethods, PyReadonlyArray3, PyReadwriteArray3, ndarray::Array1,
};
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
    // pattern match and extract allowed array types
    if let Ok(array) = i_data.extract::<PyReadonlyArray3<f32>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array();
        let output = phasor::time_domain::image(&arr, period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = i_data.extract::<PyReadonlyArray3<f64>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array();
        let output = phasor::time_domain::image(&arr, period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = i_data.extract::<PyReadonlyArray3<u16>>() {
        let ro_arr = array.readonly();
        let arr = ro_arr.as_array();
        let output = phasor::time_domain::image(&arr, period, harmonic, omega, axis);
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
    let arr = Array1::from_vec(i_data);
    phasor::time_domain::imaginary(&arr, period, harmonic, omega)
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
    let arr = Array1::from_vec(i_data);
    phasor::time_domain::real(&arr, period, harmonic, omega)
}

/// Calibrate a real and imaginary (G, S) coordinate pair.
///
/// Calibrate the real and imaginary (e.g. G and S) coordinate pair by rotating
/// and scaling by phase (φ) and modulation (M) respectively using:
///
/// g = M * cos(φ)
/// s = M * sin(φ)
/// S' = G * s + S * g
/// G' = G * g - S * s
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// :param g: The real component (G) to calibrate.
/// :param s: The imaginary component (S) to calibrate.
/// :param modulation: The modulation to scale the input (G, S) coordinates.
/// :param phi: The phi, φ, polar angle to rotate the input (G, S) coordinates.
/// :return: The calibrated coordinate pair, (G, S).
#[pyfunction]
#[pyo3(name = "coordinate_pair")]
pub fn calibration_coordinate_pair(g: f64, s: f64, modulation: f64, phi: f64) -> (f64, f64) {
    phasor::calibration::coordinate_pair(g, s, modulation, phi)
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensional phasor
/// image.
///
/// This function calibrates an input 3-dimensonal phasor image by rotating and
/// scaling G and S coordinates by phase (φ) and modulation (M) respectively using:
///
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling. This function mutates the input data and does not create a new
/// array.
///
/// :param data: The 3-dimensonal phasor image, where G and S are channels 0 and 1
///     respectively.
/// :param modulation: The modulation to scale the input (G, S) coordinates.
/// :param phi: The phi, , polar angle to rotate the intput (G, S) coorindates.
/// :param axis: The channel axis, default = 2.
#[pyfunction]
#[pyo3(name = "image_mut")]
#[pyo3(signature = (data, modulation, phi, axis=None))]
pub fn calibration_image_mut(
    mut data: PyReadwriteArray3<f64>,
    modulation: f64,
    phi: f64,
    axis: Option<usize>,
) {
    let arr = data.as_array_mut();
    phasor::calibration::image_mut(arr, modulation, phi, axis);
}

/// Compute the modulation of a multi-component phasor coordinate pair.
///
/// The modulation of a multi-component (i.e. inside the universal circle) phasor
/// coordinate pair is calculated using:
///
/// M = √(G² + S²)
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The modulation (M) of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "multi_component_modulation")]
pub fn plot_multi_component_modulation(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_modulation(g, s)
}

/// Compute the phi angle of a multi-component phasor coordinate pair.
///
/// The phi (φ) of a multi-component (i.e. inside the universal circle) phasor
/// coordinate pair is calculated using:
///
/// φ = tan⁻¹(S / G)
///
/// Computes atan(S/G) in all four quadrants using atan2.
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The phi (φ) angle of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "multi_component_phi")]
pub fn plot_multi_component_phi(g: f64, s: f64) -> f64 {
    phasor::plot::multi_component_phi(g, s)
}

/// Compute a coordinate pair for a single component decay.
///
/// This function computes a coordinate pair for a single component decay given
/// as:
///
/// G = 1 / 1 + (ωτ)²
/// S = ωτ / 1 + (ωτ)²
///
/// :param tau: The lifetime of a single component decay.
/// :param omega: The angular frequency.
/// :return: The single component decay coordinate pair, (G, S).
#[pyfunction]
#[pyo3(name = "single_component_coordinate_pair")]
pub fn plot_single_component_coordinate_pair(tau: f64, omega: f64) -> (f64, f64) {
    phasor::plot::single_component_coordinate_pair(tau, omega)
}

/// Compute the modulation of a single-component phasor coordinate pair.
///
/// The modulation (M) of a single-component (i.e. on the universal circle) phasor
/// coordinate pair is calculated using:
///
/// M = cos(φ)
///
/// :param phi: The phi (φ) angle of the (G, S) phasor coordinate pair.
/// :return: The modulation (M) of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "single_component_modulation")]
pub fn plot_single_component_modulation(phi: f64) -> f64 {
    phasor::plot::single_component_modulation(phi)
}

/// Compute the phi angle of a single-component phasor coordinate pair.
///
/// The phi (φ) angle of a single-component (i.e. on the universal circle) phasor
/// coordinate pair is calculated using:
///
///  φ = tan⁻¹(ω * τ)
///
/// :param omega: The omega (ω), angular frequency.
/// :param tau: The tau (τ), lifetime.
/// :return: The phi (φ) angle of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "single_component_phi")]
pub fn plot_single_component_phi(omega: f64, tau: f64) -> f64 {
    phasor::plot::single_component_phi(omega, tau)
}
