use numpy::{
    IntoPyArray, PyArray3, PyArrayMethods, PyReadonlyArray3, PyReadwriteArray3, ndarray::Array1,
};
use pyo3::prelude::*;

use imgal_core::phasor::{calibration, plot, time_domain};

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
/// :param phase: The phase, φ angle, to rotate the input (G, S) coordinates.
/// :return: The calibrated coordinate pair, (G, S).
#[pyfunction]
#[pyo3(name = "coordinate_pair")]
pub fn calibration_coordinate_pair(g: f64, s: f64, modulation: f64, phase: f64) -> (f64, f64) {
    calibration::coordinate_pair(g, s, modulation, phase)
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensional phasor
/// image.
///
/// # Description
///
/// This function calibrates an input 3-dimensional phasor image by rotating and
/// scaling G and S coordinates by phase (φ) and modulation (M) respectively using:
///
/// g = M * cos(φ)
/// s = M * sin(φ)
/// G' = G * g - S * s
/// S' = G * s + S * g
///
/// Where G' and S' are the calibrated real and imaginary values after rotation
/// and scaling.
///
/// This function creates a new array and does not mutate the input array.
///
/// :param data: The 3-dimensional phasor image, where G and S are channels 0
///     and 1 respectively.
/// :param modulation: The modulation to scale the input (G, S) coordinates.
/// :param phase: The phase, φ angle, to rotate the input (G, S) coordinates.
/// :param axis: The channel axis, default = 2.
/// :return: A 3-dimensional array with the calibrated phasor values, where
///     calibrated G and S are channels 0 and 1 respectively.
#[pyfunction]
#[pyo3(name = "image")]
#[pyo3(signature = (data, modulation, phase, axis=None))]
pub fn calibration_image<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    modulation: f64,
    phase: f64,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(array) = data.extract::<PyReadonlyArray3<f32>>() {
        let ro_arr = array.readonly();
        let output = calibration::image(&ro_arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray3<f64>>() {
        let ro_arr = array.readonly();
        let output = calibration::image(&ro_arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray3<u16>>() {
        let ro_arr = array.readonly();
        let output = calibration::image(&ro_arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype.",
        ));
    }
}

/// Calibrate the real and imaginary (G, S) coordinates of a 3-dimensional phasor
/// image.
///
/// This function calibrates an input 3-dimensional phasor image by rotating and
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
/// :param data: The 3-dimensional phasor image, where G and S are channels 0 and 1
///     respectively.
/// :param modulation: The modulation to scale the input (G, S) coordinates.
/// :param phase: The phase, φ angle, to rotate the intput (G, S) coorindates.
/// :param axis: The channel axis, default = 2.
#[pyfunction]
#[pyo3(name = "image_mut")]
#[pyo3(signature = (data, modulation, phase, axis=None))]
pub fn calibration_image_mut(
    mut data: PyReadwriteArray3<f64>,
    modulation: f64,
    phase: f64,
    axis: Option<usize>,
) {
    let arr = data.as_array_mut();
    calibration::image_mut(arr, modulation, phase, axis);
}

/// Find the modulation and phase calibration values.
///
/// This function calculates the modulation and phase calibration values from
/// a pair of theoretical single component coordinates (computed from "tau" and
/// "omega") and a pair of measured coordinates. The output, (M, φ), are the
/// modulation and phase values to calibrate with.
///
/// :param g: The measured real (G) value.
/// :param s: The measured imaginary (S) value.
/// :param tau: The lifetime, τ.
/// :param omega: The angular frequency, ω.
/// :param axis: The channel axis, default = 2.
/// :return: The modulation and phase calibration values, (M, φ).
#[pyfunction]
#[pyo3(name = "modulation_and_phase")]
pub fn calibration_modulation_and_phase(g: f64, s: f64, tau: f64, omega: f64) -> (f64, f64) {
    calibration::modulation_and_phase(g, s, tau, omega)
}

/// Compute the modulation of a phasor coordinate pair.
///
/// This function calculates the modulation (M) of a phasor coordinate pair
/// using the pythagorean theorem to find the hypotenuse (i.e. the modulation):
///
/// M = √(G² + S²)
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The modulation (M) of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "modulation")]
pub fn plot_modulation(g: f64, s: f64) -> f64 {
    plot::modulation(g, s)
}

/// Compute the phase of a phasor coordinate pair.
///
/// This function calculates the phase or phi (φ) of a phasor coordinate pair
/// using:
///
/// φ = tan⁻¹(S / G)
///
/// This implementation uses atan2 and computes the four quadrant arctanget of
/// the phasor coordinate pair.
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The phase (phi, φ) angle of the (G, S) phasor coordinate pair.
#[pyfunction]
#[pyo3(name = "phase")]
pub fn plot_phase(g: f64, s: f64) -> f64 {
    plot::phase(g, s)
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
    plot::single_component_coordinate_pair(tau, omega)
}

/// Compute the real and imaginary (G, S) coordinates of a 3-dimensional decay
/// image.
///
/// The real (G) and imaginary (S) components are calculated using the normalized
/// sine and cosine Fourier transforms:
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// :param data: I(t), the decay data image.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :param axis: The decay or lifetime axis, default = 2.
/// :return: The real and imaginary coordinates as a 3-dimensional (row, col, ch)
///     image, where G and S are indexed at 0 and 1 respectively on the channel axis.
#[pyfunction]
#[pyo3(name = "image")]
#[pyo3(signature = (data, period, harmonic=None, omega=None, axis=None))]
pub fn time_domain_image<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(array) = data.extract::<PyReadonlyArray3<f32>>() {
        let ro_arr = array.readonly();
        let output = time_domain::image(&ro_arr.as_array(), period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray3<f64>>() {
        let ro_arr = array.readonly();
        let output = time_domain::image(&ro_arr.as_array(), period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(array) = data.extract::<PyReadonlyArray3<u16>>() {
        let ro_arr = array.readonly();
        let output = time_domain::image(&ro_arr.as_array(), period, harmonic, omega, axis);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported array dtype.",
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
/// :param data: I(t), the 1-dimensional decay curve.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :return: The imaginary component, S.
#[pyfunction]
#[pyo3(name = "imaginary")]
#[pyo3(signature = (data, period, harmonic=None, omega=None))]
pub fn time_domain_imaginary(
    data: Vec<f64>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    let arr = Array1::from_vec(data);
    time_domain::imaginary(&arr, period, harmonic, omega)
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
/// :param data: I(t), the 1-dimensional decay curve.
/// :param period: The period.
/// :param harmonic: The harmonic value, default = 1.0.
/// :param omega: The angular frequency.
/// :return: The real component, G.
#[pyfunction]
#[pyo3(name = "real")]
#[pyo3(signature = (data, period, harmonic=None, omega=None))]
pub fn time_domain_real(
    data: Vec<f64>,
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    let arr = Array1::from_vec(data);
    time_domain::real(&arr, period, harmonic, omega)
}
