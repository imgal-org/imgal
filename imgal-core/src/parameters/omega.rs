use std::f64::consts;

/// Compute the angular frequency (Omega) value.
///
/// This function computes the angular frequency, Omega,
/// using the following equation:
///
/// ω = 2π/T
///
/// # Arguments
///
/// * `period` - The period in seconds.
///
/// # Returns
///
/// The Omega value.
pub fn omega(period: f64) -> f64 {
    (2.0 * consts::PI) / period
}
