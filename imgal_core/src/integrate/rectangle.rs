use crate::statistics::sum;

/// Perform numerical integration using the midpoint rule.
///
/// # Description
///
/// Approximates the definite integral using the Midpoint rule
/// with pre-computed y-values.
///
/// ∫\[a,b\] f(x) dx ≈ h × [f(x₁) + f(x₂) + ... + f(xₙ)]
///
/// # Arguments
///
/// * `y`: Slice of the data to integrate.
/// * `h`: The width between data points, default = 1.0.
///
/// # Returns
///
/// * `f64`: The computed integral.
pub fn midpoint(y: &[f64], h: Option<f64>) -> f64 {
    h.unwrap_or(1.0) * sum(y)
}
