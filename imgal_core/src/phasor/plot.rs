use std::f64;

/// Compute the modulation of a multi-component phasor coordinate.
///
/// # Description
///
/// The modulation of a multi-component phasor coordinate is calculated
/// using:
///
/// M = √(G² + S²)
///
/// # Arguments
///
/// * `g`: The real, G, component.
/// * `s`: The imaginary, S, component.
///
/// # Returns
///
/// * `f64`: The modulation of the G and S phasor coordinates.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the phi polar angle of a multi-component phasor coordinate.
///
/// # Description
///
/// The phi, φ, of a multi-component phasor coordinate is calculated
/// using:
///
/// φ = tan⁻¹(S / G)
///
/// Computes atan(S/G) in all four quadrants using atan2.
///
/// # Arguments
///
/// * `g`: The real, G, component.
/// * `s`: The imaginary, S, component.
///
/// # Returns
///
/// * `f64`: The phi, φ, polar angle of the G and S phasor coordinate.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_phi(g: f64, s: f64) -> f64 {
    s.atan2(g)
}

/// Compute the modulation of a single-component phasor coordinate.
///
/// # Description
///
/// The modulation of a single-component phasor coordinate is calculated
/// using:
///
/// M = cos(φ)
///
/// # Arguments
///
/// * `phi`: The phi, φ, polar angle of the phasor coordinate.
///
/// # Returns
///
/// * `f64`: Modulation of a single-component phasor (g, s) coordiate.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_modulation(phi: f64) -> f64 {
    f64::cos(phi)
}

/// Compute the phi polar angle of a single-component phasor coordinate.
///
/// # Description
///
/// The phi, φ, polar angle of a single-component phasor coordinate is
/// calculated using:
///
///  φ = tan⁻¹(ω * τ)
///
/// # Arguments
///
/// * `omega`: The omega, ω, angular frequency in radians.
/// * `tau`: The tau, τ, lifetime in seconds.
///
/// # Returns
///
/// * `f64`: Phi, φ, the polar angle of a single-component phasor (g, s) coordinate.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_phi(omega: f64, tau: f64) -> f64 {
    (omega * tau).atan()
}
