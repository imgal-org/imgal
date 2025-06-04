use std::f64::consts;

/// Compute the angular frequency (omega) value.
///
/// # Description
///
/// This function computes the angular frequency, omega (ω),
/// using the following equation:
///
/// ω = 2π/T
///
/// Where T is the period.
///
/// # Arguments
///
/// * `period`: The period in seconds.
///
/// # Returns
///
/// * `f64`: The omega (ω) value.
pub fn omega<T: Into<f64>>(period: T) -> f64
{
    2.0 * consts::PI / T::into(period)
}
