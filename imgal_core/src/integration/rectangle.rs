use crate::statistics::sum;

/// Integrate a curve with the midpoint rule.
///
/// # Description
///
/// Approximates the definite integral using the midpoint rule
/// with pre-computed x-values:
///
/// ∫f(x) dx ≈ Δx * [f(x₁) + f(x₂) + ... + f(xₙ)]
///
/// # Arguments
///
/// * `x`: The 1-dimensional data to integrate.
/// * `delta_x`: The width between data points, default = 1.0.
///
/// # Returns
///
/// * `f64`: The computed integral.
pub fn midpoint(x: &[f64], delta_x: Option<f64>) -> f64 {
    delta_x.unwrap_or(1.0) * sum(x)
}
