use ndarray::Array1;

/// Simulate a 1-dimensional fluorescence decay curve.
///
/// # Description
///
/// A fluorescence decay curve is computed as:
///
/// I(t) = Io * e^(-t/τ)
///
/// Where "Io" is the initial fluorescence value and "t" is time (the `bins`
/// parameter).
///
/// # Arguments
/// 
/// * `samples`: The number of descrete points that make up the decay curve (i.e. time).
/// * `period`: The period in the same unit as tau (e.g. nanoseconds).
/// * `tau`: The lifetime in the same unit as the period (e.g. nanoseconds).
/// * `initial_value`: The initial fluorescence value, default=0.0.
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
