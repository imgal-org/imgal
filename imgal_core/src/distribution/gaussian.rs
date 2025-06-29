use ndarray::Array1;

use crate::statistics::sum;

/// Generate a normalized Gaussian distribution over a specified range.
///
/// # Description
///
/// This function creates a discrete Gaussian distribution by sampling the continuous
/// Gaussian probability density function at evenly spaced points across a given range.
/// The resulting distribution is normalized so that all values sum to 1.0.i
/// The function implements the Gaussian probability density function:
///
/// ```text
/// f(x) = exp(-((x - μ)² / (2σ²)))
/// ```
/// where:
/// - `x` is the position along the range
/// - `μ` is the center (mean)
/// - `σ` is the sigma (standard deviation)
///
/// # Arguments
///
/// * `sigma`: The standard deviation of the Gaussian distribution (_i.e._ the width).
/// * `bins`: The number of discrete points to sample the Gaussian distribution.
/// * `range`: The total width of the sampling range.
/// * `center`: The mean (center) of the Gaussian distribution (_i.e._ the peak).
///
/// # Returns
///
/// * `Array<f64>`: The normalized Gaussian distribution.
pub fn gaussian(sigma: f64, bins: usize, range: f64, center: f64) -> Array1<f64> {
    // create data range (i.e. time) and gaussian arrays
    let mut r = Array1::<f64>::zeros(bins);
    let mut g = Array1::<f64>::zeros(bins);

    // calculate range data
    let width = range / (bins as f64 - 1.0);
    r.iter_mut().enumerate().for_each(|(i, v)| {
        *v = i as f64 * width;
    });

    // calculate the gaussian distrubtion
    let sigma_sq_2 = 2.0 * sigma.powi(2);
    g.iter_mut().enumerate().for_each(|(i, v)| {
        *v = (-((r[i] - center).powi(2)) / sigma_sq_2).exp();
    });

    // normalize the gaussian distribution
    let g_sum = sum(g.as_slice().unwrap());
    g.iter_mut().for_each(|v| {
        *v /= g_sum;
    });
    g
}
