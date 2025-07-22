use ndarray::{Array1, Array3};

use crate::filters::fft_convolve;
use crate::simulation::instrument;

/// Simulate a 1-dimensional gaussian IRF convolved decay curve.
///
/// # Description
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (1-dimensional) by FFT convolving the IRF with a decay cruve. The ideal
/// decay curve is computed as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// The ideal decay curve is then convolved with a Guassian IRF.
///
/// # Arguments
///
/// * `samples`: The number of descrete points that make up the decay curve (_i.e._ time).
/// * `period`: The period, in the same unit as thee other parameters(_e.g._ seconds).
/// * `tau`: The lifetime, in the same unit as the other parameters (_e.g._ seconds).
/// * `initial_value`: The initial fluorescence value.
/// * `irf_width`: The full width at half maximum (FWHM) of the IRF.
/// * `irf_center`: The temporal position of the IRF peak within the time range.
///
/// # Returns
///
/// * `Vec<f64>`: The 1-dimensional Gaussian IRF convolved decay curve.
pub fn gaussian_fluorescence_1d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
) -> Array1<f64> {
    let irf = instrument::gaussian_irf_1d(samples, period, irf_width, irf_center);
    let decay = ideal_fluorescence_1d(samples, period, tau, initial_value);
    fft_convolve(decay.view(), irf.view())
}

/// Simulate a 3-dimensional Gaussian IRF convolved decay curve.
///
/// # Description
///
/// Compute a Gaussian instrument response function (IRF) convolved curve
/// (3-dimensional) by FFT convolving the IRF with a decay cruve. The ideal
/// decay curve is computed as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// The ideal decay curve is then convolved with a Guassian IRF.
///
/// # Arguments
///
/// * `samples`: The number of descrete points that make up the decay curve (_i.e._ time).
/// * `period`: The period, in the same unit as thee other parameters(_e.g._ seconds).
/// * `tau`: The lifetime, in the same unit as the other parameters (_e.g._ seconds).
/// * `initial_value`: The initial fluorescence value.
/// * `irf_width`: The full width at half maximum (FWHM) of the IRF.
/// * `irf_center`: The temporal position of the IRF peak within the time range.
/// * `shape`: The row and col shape to broadcast the decay curve into.
///
/// # Returns
///
/// * `Array3<f64>`: The 3-dimensional Gaussian IRF convolved decay curve.
pub fn gaussian_fluorescence_3d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    irf_width: f64,
    irf_center: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional gaussian IRF convolved curve and broadcast
    let d = gaussian_fluorescence_1d(samples, period, tau, initial_value, irf_width, irf_center);
    let new_shape = (shape.0, shape.1, samples);
    d.broadcast(new_shape).unwrap().to_owned()
}

/// Simulate an ideal 1-dimensional fluorescence decay curve.
///
/// # Description
///
/// A fluorescence decay curve is computed as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// Where "Io" is the initial fluorescence value and "t" is time (_i.e._ the
/// number of samples).
///
/// # Arguments
///
/// * `samples`: The number of descrete points that make up the decay curve (i.e. time).
/// * `period`: The period in the same unit as tau (e.g. nanoseconds).
/// * `tau`: The lifetime in the same unit as the period (e.g. nanoseconds).
/// * `initial_value`: The initial fluorescence value.
///
/// # Returns
///
/// * `Array1<f64>`: The 1-dimensional decay curve.
///
/// # Reference
///
/// <https://doi.org/10.1111/j.1749-6632.1969.tb56231.x>
pub fn ideal_fluorescence_1d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> Array1<f64> {
    // create time array and compute the decay curve
    let t: Array1<f64> = Array1::linspace(0.0, period, samples);
    t.map(|ti| initial_value * (-ti / tau).exp())
}

/// Simulate an ideal 3-dimensional fluorescence decay curve.
///
/// A fluorescence decay curve is computed as:
///
/// ```text
/// I(t) = Io * e^(-t/τ)
/// ```
///
/// Where "Io" is the initial fluorescence value and "t" is the time (_i.e._ the
/// number of samples). The decay curve is then broadcasted to the specified input
/// shape with the number of samples along the last axis.
///
/// # Arguments
///
/// * `samples`: The number of descrete points that make up the decay curve (i.e. time).
/// * `period`: The period in the same unit as tau (e.g. nanoseconds).
/// * `tau`: The lifetime in the same unit as the period (e.g. nanoseconds).
/// * `initial_value`: The initial fluorescence value.
/// * `shape`: The row and col shape to broadcast the decay curve into.
///
/// # Return
///
/// * `Array3<f64>`: The 3-dimensional decay curve.
///
/// # Reference
///
/// <https://doi.org/10.1111/j.1749-6632.1969.tb56231.x>
pub fn ideal_fluorescence_3d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional decay curve and broadcast
    let d = ideal_fluorescence_1d(samples, period, tau, initial_value);
    let new_shape = (shape.0, shape.1, samples);
    d.broadcast(new_shape).unwrap().to_owned()
}
