use ndarray::Array1;

use imgal_core::phasor::time_domain;

/// Helper function to create phasor test data.
///
/// # Description
///
/// This helper function creates a 1D array of exponential decay
/// test data, where decay is calculated as:
///
/// I(t) = Io * e^(-t/τ)
///
/// # Returns
///
/// A tuple containing:
/// - `Array1<f64>`: An ideal, noise free, decay curve.
/// - `f64`: The period.
fn create_phasor_test_data() -> (Array1<f64>, f64) {
    // test data parameters
    let samples: usize = 128;
    let period: f64 = 0.04;
    let ii: f64 = 10.0;
    let tau: f64 = 1.0;

    // create time array and compute the decay curve
    let t: Array1<f64> = Array1::linspace(0.0, (samples as f64 - 1.0) * period, samples);
    let decay: Array1<f64> = t.mapv(|ti| ii * (-ti / tau).exp());

    // return decay array and period
    (decay, period)
}

#[test]
fn test_time_domain_imaginary() {
    let (data, period) = create_phasor_test_data();
    let s: f64 = time_domain::imaginary(data.as_slice().unwrap(), period, None, None);
    assert_eq!(s, 0.47987693150630056);
}

#[test]
fn test_time_domain_real() {
    let (data, period) = create_phasor_test_data();
    let g: f64 = time_domain::real(data.as_slice().unwrap(), period, None, None);
    assert_eq!(g, 0.4109055941275861);
}
