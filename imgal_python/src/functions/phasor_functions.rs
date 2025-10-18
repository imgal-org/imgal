use numpy::{
    IntoPyArray, PyArray2, PyArray3, PyReadonlyArray2, PyReadonlyArray3, PyReadwriteArray3,
};
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

use crate::error::map_array_error;
use imgal::phasor::{calibration, plot, time_domain};

/// Calibrate a real and imaginary (G, S) coordinates.
///
/// Calibrate the real and imaginary (e.g. G and S) coordinates by rotating
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
/// :return: The calibrated coordinates, (G, S).
#[pyfunction]
#[pyo3(name = "coordinates")]
pub fn calibration_coordinates(g: f64, s: f64, modulation: f64, phase: f64) -> (f64, f64) {
    calibration::coordinates(g, s, modulation, phase)
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
    if let Ok(arr) = data.extract::<PyReadonlyArray3<u8>>() {
        let output = calibration::image(arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<u16>>() {
        let output = calibration::image(arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f32>>() {
        let output = calibration::image(arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f64>>() {
        let output = calibration::image(arr.as_array(), modulation, phase, axis);
        return Ok(output.into_pyarray(py));
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
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
/// theoretical monoexponential coordinates (computed from "tau" and
/// "omega") and measured coordinates. The output, (M, φ), are the
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

/// Compute the modulation of phasor G and S coordinates.
///
/// This function calculates the modulation (M) of phasor G and S coordinates
/// using the pythagorean theorem to find the hypotenuse (i.e. the modulation):
///
/// M = √(G² + S²)
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The modulation (M) of the (G, S) phasor coordinates.
#[pyfunction]
#[pyo3(name = "modulation")]
pub fn plot_modulation(g: f64, s: f64) -> f64 {
    plot::modulation(g, s)
}

/// Compute the phase of phasor G and S coordinates.
///
/// This function calculates the phase or phi (φ) of phasor G and S coordinates
/// using:
///
/// φ = tan⁻¹(S / G)
///
/// This implementation uses atan2 and computes the four quadrant arctanget of
/// the phasor coordinates.
///
/// :param g: The real component, G.
/// :param s: The imaginary component, S.
/// :return: The phase (phi, φ) angle of the (G, S) phasor coordinates.
#[pyfunction]
#[pyo3(name = "phase")]
pub fn plot_phase(g: f64, s: f64) -> f64 {
    plot::phase(g, s)
}

/// Compute the G and S coordinates for a monoexponential decay.
///
/// This function computes the G and S coordinates for a monoexponential decay
/// given as:
///
/// G = 1 / 1 + (ωτ)²
/// S = ωτ / 1 + (ωτ)²
///
/// :param tau: The lifetime of a monoexponential.
/// :param omega: The angular frequency.
/// :return: The single component decay coordinates, (G, S).
#[pyfunction]
#[pyo3(name = "monoexponential_coordinates")]
pub fn plot_monoexponential_coordinates(tau: f64, omega: f64) -> (f64, f64) {
    plot::monoexponential_coordinates(tau, omega)
}

/// Map G and S coordinates back to the input phasor array as a boolean mask.
///
/// This function maps the G and S coordinates back to the input G/S phasor
/// array and returns a 2-dimensional boolean mask where "true" indicates
/// G and S coordiantes presentin the "g_coords" and "s_coords" arrays.
///
/// :param data: The G/S 3-dimensional array.
/// :param g_coords: A 1-dimensional array of "g" coordinates in the "data" array.
///     The "g_coords" and "s_coords" array lengths must match.
/// :param s_coords: A 1-dimensional array of "s" coordiantes in the "data" array.
/// *   The "s_coords" and "g_coords" array lengths must match.
/// :param axis: The channel axis, default = 2.
/// :return: A 2-dimensional boolean mask where "true" pixels
///     represent values found in the "g_coords" and "s_coords" arrays.
#[pyfunction]
#[pyo3(name = "map_mask")]
#[pyo3(signature = (data, g_coords, s_coords, axis=None))]
pub fn plot_map_mask<'py>(
    py: Python<'py>,
    data: PyReadonlyArray3<f64>,
    g_coords: Vec<f64>,
    s_coords: Vec<f64>,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray2<bool>>> {
    plot::map_mask(data.as_array(), &g_coords, &s_coords, axis)
        .map(|output| output.into_pyarray(py))
        .map_err(map_array_error)
}

/// Compute the histogram quality value from a 1-dimensional decay array.
///
/// This function computes a weighted quality metric, "q", for time domain
/// fluorescence lifetime imaging microscopy (FLIM) photon arrival histograms.
/// Bins exceeding the "count_threshold" contribute a weighted value to "q",
/// rewarding bins with high counts, while still considering histograms with
/// their counts distributed across multiple valid bins. The "q" value provides
/// a measure of photon counts above threshold and the count distribution
/// quality. The histogram quality metric is defined as:
///
/// q = (vb / n²) * ∑₍xᵢ > t₎ xᵢ²
///
/// where:
/// - "n" is the total number of bins.
/// - "xᵢ" is the value in bin "i".
/// - "vb" is the number of valid bins (_i.e._ bins above threshold).
/// - "t" is the count threshold.
/// - "I(·)" is the indicator function, returns "true" if "xᵢ" is above the
///    threshold, else it returns "false".
///
/// :param data: The 1-dimensional decay data as a slice.
/// :param count_threshold: The minimum bin count value a bin must exceed to be
///     considered valid.
/// :return: The histogram quality value, "q". A "q" value of 0.0 occurs when
///     no bins exceed the threshold, indicating an insufficient histogram for
///     analysis. Values in the range of < 1.0 indicate histograms with few
///     photons overall and/or photons disributed across many other bins with
///     low counts per bin. Values between 1.0 and 10.0 contain good photon counts
///     and/or photon distribution across the histogram. Values exceeding 10.0
///     indicate high quality histograms with high photon counts that are
///     distributed across the histogram.
#[pyfunction]
#[pyo3(name = "histogram_quality")]
pub fn time_domain_histogram_quality(data: Vec<f64>, count_threshold: f64) -> f64 {
    time_domain::histogram_quality(&data, count_threshold)
}

/// Compute a histogram quality image from a 3-dimensional decay array.
///
/// This function computes a weighted quality metric, "q", for time domain
/// fluorescence lifetime imaging microscopy (FLIM) photon arrival 3-dimensional
/// arrays. Bins exceeding the "count_threshold" contribute a weighted value to
/// "q", rewarding bins with high counts, while still considering histograms
/// with their counts distributed across multiple valid bins. The "q" value
/// provides a measure of photon counts above threshold and the count
/// distribution quality. This function returns the "q" values as a
/// 2-dimensional array or "q" map. The histogram quality metric is defined as:
///
/// q = ∑ (I(xᵢ > t) * xᵢ² * vb) / n²
///
/// where:
/// - "n" is the total number of bins.
/// - "xᵢ" is the value in bin "i".
/// - `vb` is the number of valid bins (_i.e._ bins above threshold).
/// - "t" is the count threshold.
/// - "I(·)" is the indicator function, returns "true" if "xᵢ" is above the
///    threshold, else it returns "false".
///
/// :param data: The 3-dimensional decay data.
/// :param count_threshold: The minimum bin count value a bin must exceed to be
///     considered valid.
/// :param axis: The decay or lifetime axis, default = 2.
/// :return: The 2-dimensional pixel-wise histogram quality, "q",
///     value map. A "q" value of 0.0 occurs when no bins exceed the threshold,
///     indicating an insufficient histogram for analysis. Values in the range
///     of < 1.0 indicate histograms with few photons overall and/or photons
///     disributed across many other bins with low counts per bin. Values
///     between 1.0 and 10.0 contain good photon counts and/or photon
///     distribution across the histogram. Values exceeding 10.0 indicate high
///     quality histograms with high photon counts that are distributed across
///     the histogram.
#[pyfunction]
#[pyo3(name = "histogram_quality_image")]
#[pyo3(signature = (data, count_threshold, axis=None))]
pub fn time_domain_histogram_quality_image<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    count_threshold: f64,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray2<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(arr) = data.extract::<PyReadonlyArray3<u8>>() {
        return time_domain::histogram_quality_image(arr.as_array(), count_threshold as u8, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error);
    }
    if let Ok(arr) = data.extract::<PyReadonlyArray3<u16>>() {
        return time_domain::histogram_quality_image(arr.as_array(), count_threshold as u16, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error);
    }
    if let Ok(arr) = data.extract::<PyReadonlyArray3<f32>>() {
        return time_domain::histogram_quality_image(arr.as_array(), count_threshold as f32, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error);
    }
    if let Ok(arr) = data.extract::<PyReadonlyArray3<f64>>() {
        return time_domain::histogram_quality_image(arr.as_array(), count_threshold, axis)
            .map(|output| output.into_pyarray(py))
            .map_err(map_array_error);
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
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
/// :param axis: The decay or lifetime axis, default = 2.
/// :return: The real and imaginary coordinates as a 3-dimensional (row, col, ch)
///     image, where G and S are indexed at 0 and 1 respectively on the channel axis.
#[pyfunction]
#[pyo3(name = "image")]
#[pyo3(signature = (data, period, mask=None, harmonic=None, axis=None))]
pub fn time_domain_image<'py>(
    py: Python<'py>,
    data: Bound<'py, PyAny>,
    period: f64,
    mask: Option<PyReadonlyArray2<bool>>,
    harmonic: Option<f64>,
    axis: Option<usize>,
) -> PyResult<Bound<'py, PyArray3<f64>>> {
    // pattern match and extract allowed array types
    if let Ok(arr) = data.extract::<PyReadonlyArray3<u8>>() {
        if let Some(m) = mask {
            return time_domain::image(arr.as_array(), period, Some(m.as_array()), harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        } else {
            return time_domain::image(arr.as_array(), period, None, harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        }
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<u16>>() {
        if let Some(m) = mask {
            return time_domain::image(arr.as_array(), period, Some(m.as_array()), harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        } else {
            return time_domain::image(arr.as_array(), period, None, harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        }
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f32>>() {
        if let Some(m) = mask {
            return time_domain::image(arr.as_array(), period, Some(m.as_array()), harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        } else {
            return time_domain::image(arr.as_array(), period, None, harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        }
    } else if let Ok(arr) = data.extract::<PyReadonlyArray3<f64>>() {
        if let Some(m) = mask {
            return time_domain::image(arr.as_array(), period, Some(m.as_array()), harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        } else {
            return time_domain::image(arr.as_array(), period, None, harmonic, axis)
                .map(|output| output.into_pyarray(py))
                .map_err(map_array_error);
        }
    } else {
        return Err(PyErr::new::<PyTypeError, _>(
            "Unsupported array dtype, supported array dtypes are u8, u16, f32, and f64.",
        ));
    }
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
/// :return: The imaginary component, S.
#[pyfunction]
#[pyo3(name = "imaginary")]
#[pyo3(signature = (data, period, harmonic=None))]
pub fn time_domain_imaginary(data: Vec<f64>, period: f64, harmonic: Option<f64>) -> f64 {
    time_domain::imaginary(&data, period, harmonic)
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
/// :return: The real component, G.
#[pyfunction]
#[pyo3(name = "real")]
#[pyo3(signature = (data, period, harmonic=None))]
pub fn time_domain_real(data: Vec<f64>, period: f64, harmonic: Option<f64>) -> f64 {
    time_domain::real(&data, period, harmonic)
}
