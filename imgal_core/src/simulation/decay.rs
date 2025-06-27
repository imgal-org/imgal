use ndarray::{Array1, Array3};

/// Simulate a 1-dimensional fluorescence decay curve.
///
/// # Description
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
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
pub fn fluorescence_decay_1d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
) -> Array1<f64> {
    // create time array and compute the decay curve
    let t: Array1<f64> = Array1::linspace(0.0, (samples as f64 - 1.0) * period, samples);
    t.map(|ti| initial_value * (-ti / tau).exp())
}

/// Simulate a 3-dimensional fluorescence decay curve.
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
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
pub fn fluorescence_decay_3d(
    samples: usize,
    period: f64,
    tau: f64,
    initial_value: f64,
    shape: (usize, usize),
) -> Array3<f64> {
    // create 1-dimensional decay curve and broadcast
    let d = fluorescence_decay_1d(samples, period, tau, initial_value);
    let new_shape = (shape.0, shape.1, samples);
    d.broadcast(new_shape).unwrap().to_owned()
}
