use std::f64;

/// Compute the modulation of phasor G and S coordinates.
///
/// # Description
///
/// This function calculates the modulation (M) of phasor G and S coordinates
/// using the pythagorean theorem to find the hypotenuse (_i.e._ the modulation):
///
/// ```text
/// M = √(G² + S²)
/// ````
///
/// # Arguments
///
/// * `g`: The real component, G.
/// * `s`: The imaginary component, S.
///
/// # Returns
///
/// * `f64`: The modulation (M) of the (G, S) phasor coordinates.
///
pub fn modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the phase of phasor G and S coordinates.
///
/// # Description
///
/// This function calculates the phase or phi (φ) of phasor G and S coordinates
/// using:
///
/// ```text
/// φ = tan⁻¹(S / G)
/// ````
///
/// This implementation uses atan2 and computes the four quadrant arctangent of
/// the phasor coordinates.
///
/// # Arguments
///
/// * `g`: The real component, G.
/// * `s`: The imaginary component, S.
///
/// # Returns
///
/// * `f64`: The phase (phi, φ)  of the (G, S) phasor coordinates.
pub fn phase(g: f64, s: f64) -> f64 {
    s.atan2(g)
}

/// Compute the G and S coordinates for a monoexponential decay.
///
/// # Description
///
/// This function computes the G and S coordinates for a monoexponential decay
/// given as:
///
/// ```text
/// G = 1 / 1 + (ωτ)²
/// S = ωτ / 1 + (ωτ)²
/// ```
///
/// # Arguments
///
/// * `tau`: The lifetime of a monoexponential decay.
/// * `omega`: The angular frequency.
///
/// # Returns
///
/// * `(f64, f64)`: The monoexponential decay coordinates, (G, S).
///
/// # Reference
///
/// <https://doi.org/10.1117/1.JBO.25.7.071203>
pub fn monoexponential_coordinates(tau: f64, omega: f64) -> (f64, f64) {
    let denom = 1.0 + (omega * tau).powi(2);
    let g = 1.0 / denom;
    let s = (omega * tau) / denom;
    (g, s)
}
