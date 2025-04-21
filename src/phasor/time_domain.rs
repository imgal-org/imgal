use std::f64;

use crate::integrate::midpoint;
use crate::parameters;

/// Compute the imaginary S component of lifetime data.
///
/// The imaginary component, S, time domain equation is calculated
/// using:
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `i_data` - I(t), the decay data slice.
/// * `period` - The period in seconds.
/// * `harmonic` - The harmonic value, default = 1.0.
/// * `omega` - The angular frequency, default = computed from the period.
///
/// # Returns
///
/// * `f64` - The imaginary, S, component.
pub fn imaginary(
    i_data: &[f64],
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(|| parameters::omega(period));

    // integrate sine transform (imaginary)
    let n: usize = i_data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(i_data[i] * f64::sin(h_w_dt * (i as f64)));
    }
    let i_sin_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(&i_data, Some(dt));
    i_sin_integral / i_integral
}


/// Compute the real G component of lifetime data.
///
/// The real component, G, time domain equation is calculated
/// using:
///
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `i_data` - I(t), the decay data slice.
/// * `period` - The period in seconds.
/// * `harmonic` - The harmonic value, default = 1.0.
/// * `omega` - The angular frequency, default = computed from the period.
///
/// # Returns
///
/// * `f64` - The real, G, component.
pub fn real(
    i_data: &[f64],
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
) -> f64 {
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(|| parameters::omega(period));

    // integrate cosine transform (real)
    let n: usize = i_data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(i_data[i] * f64::cos(h_w_dt * (i as f64)));
    }
    let i_cos_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(&i_data, Some(dt));
    i_cos_integral / i_integral
}
