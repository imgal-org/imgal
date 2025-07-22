use std::f64;

/// Compute the modulation of a phasor coordinate pair.
///
/// # Description
///
/// This function calculates the modulation (M) of a phasor coordinate pair
/// using the pythagorean theorem to find the hypotenuse (i.e. the modulation):
///
/// M = √(G² + S²)
///
/// # Arguments
///
/// * `g`: The real component, G.
/// * `s`: The imaginary component, S.
///
/// # Returns
///
/// * `f64`: The modulation (M) of the (G, S) phasor coordinate pair.
///
pub fn modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the phase of a phasor coordinate pair.
///
/// # Description
///
/// This function calculates the phase or phi (φ) of a phasor coordinate pair
/// using:
///
/// φ = tan⁻¹(S / G)
///
/// This implementation uses atan2 and computes the four quadrant arctangent of
/// the phasor coordinate pair.
///
/// # Arguments
///
/// * `g`: The real component, G.
/// * `s`: The imaginary component, S.
///
/// # Returns
///
/// * `f64`: The phase (phi, φ)  of the (G, S) phasor coordinate pair.
pub fn phase(g: f64, s: f64) -> f64 {
    s.atan2(g)
}

/// Compute a coordinate pair for a single component decay.
///
/// # Description
///
/// The function computes a coordinate pair for a single component decay given
/// as:
///
/// G = 1 / 1 + (ωτ)²\
/// S = ωτ / 1 + (ωτ)²
///
/// # Arguments
///
/// * `tau`: The lifetime of a single component decay.
/// * `omega`: The angular frequency.
///
/// # Returns
///
/// * `(f64, f64)`: The single component decay coordinate pair, (G, S).
///
/// # Reference
///
/// <https://doi.org/10.1117/1.JBO.25.7.071203>
pub fn single_component_coordinate_pair(tau: f64, omega: f64) -> (f64, f64) {
    let denom = 1.0 + (omega * tau).powi(2);
    let g = 1.0 / denom;
    let s = (omega * tau) / denom;
    (g, s)
}
