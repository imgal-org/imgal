/// Perform numerical integration using Simpson's 1/3 rule
/// and the trapizoid rule (for odd number subintervals).
///
/// # Description
///
/// Simpson's 1/3 rule apporximates the integral of a function
/// with an even number of subintervals using:
/// ∫(f(x)dx) ≈ (Δx/3) * [f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + 2f(xₙ₋₂) + 4f(xₙ₋₁) + f(xₙ)]
/// Where "n" is the number of evenly spaced points in the data.
///
/// If there is an odd number of subintervals, the final
/// subinterval is integrated using the trapezoid rule:
/// ∫(f(x)dx) ≈ (Δx/2) * [f(x₀) + f(x₁)]
///
/// # Arguments
///
/// * `y` - Slice of the data to integrate.
/// * `delta_x` - The width between data points.
///
/// # Returns
///
/// * `f64` - The computed integral.
pub fn composite_simpson(y: &[f64], delta_x: Option<f64>) -> f64 {
    // set default delta x if necessary
    let d_x: f64 = delta_x.unwrap_or(1.0);
    // find the number of subintervals
    let n: usize = y.len() - 1;
    // check for even number of subintervals
    if n % 2 == 0 {
        simpson(y, delta_x).unwrap()
    } else {
        // compute the even subintervals with Simpson's rule
        let integral: f64 = simpson(&y[..n], delta_x).unwrap();
        // compute the last subinterval with a trapizoid
        let trap: f64 = (d_x / 2.0) * (y[n - 1] + y[n]);
        integral + trap
    }
}

/// Perform numerical integration using Simpson's 1/3 rule.
///
/// Simpson's 1/3 rule apporximates the integral of a function
/// with an even number of subintervals using:
/// ∫(f(x)dx) ≈ (Δx/3) * [f(x₀) + 4f(x₁) + 2f(x₂) + 4f(x₃) + ... + 2f(xₙ₋₂) + 4f(xₙ₋₁) + f(xₙ)]
///
/// Where "n" is the number of evenly spaced points in the data.
///
/// # Arguments
///
/// * `y` - Slice of the data to integrate.
/// * `delta_x` - The width between data points.
///
/// # Returns
///
/// * `Ok(f64)` - The computed integral.
/// * `Err(&str)` - Error message if the number of subintervals is odd
pub fn simpson(y: &[f64], delta_x: Option<f64>) -> Result<f64, &'static str> {
    // set default delta x if necessary
    let d_x: f64 = delta_x.unwrap_or(1.0);
    // find the number of subintervals
    let n: usize = y.len() - 1;
    // check for even number of subintervals
    if n % 2 == 0 {
        // compute integal with Simpson's rule
        let mut coef: f64;
        let mut integral: f64 = y[0] + y[n];
        for i in 1..n {
            coef = if i % 2 == 1 { 4.0 } else { 2.0 };
            integral += coef * y[i];
        }
        Ok((d_x / 3.0) * integral)
    } else {
        Err("Odd number of subintervals is not supported for Simpson's 1/3 rule.")
    }
}
