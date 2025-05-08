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
/// * `g` - The real, G, component.
/// * `s` - The imaginary, S, component.
///
/// # Returns
///
/// * `f64` - The modulation of the G and S phasor coordinates.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the theta of a multi-component phasor coordinate.
///
/// # Description
///
/// The theta of a multi-component phasor coordinate is calculated
/// using:
///
/// θ = tan⁻¹(S / G)
///
/// Computes atan(S / G) in all four quadrants using atan2.
///
/// # Arguments
///
/// * `g` - The real, G, component.
/// * `s` - The imaginary, S, component.
///
/// # Returns
///
/// * `f64` - The theta angle of the G and S phasor coordinates.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_theta(g: f64, s: f64) -> f64 {
    s.atan2(g)
}

/// Compute the modulation of a single-component phasor coordinate.
///
/// # Description
///
/// The modulation of a single-component phasor coordinate is calculated
/// using:
///
/// M = cos(θ)
///
/// # Arguments
///
/// * `theta` - The theta, θ, angle of the phasor coordinate.
///
/// # Returns
///
/// * `f64` - Modulation of a single-component phasor (g, s) coordiate.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_modulation(theta: f64) -> f64 {
    f64::cos(theta)
}

/// Compute the theta of a single-component phasor coordinate.
///
/// # Description
///
/// The theta of a single-component phasor coordinate is calculated
/// using:
///
/// θ = tan⁻¹(ω * τ)
///
/// # Arguments
///
/// * `omega` - The omega, ω, angular frequency in radians.
/// * `tau` - The tau, τ, lifetime in seconds.
///
/// # Returns
///
/// * `f64` - Theta, θ, of a single-component phasor (g, s) coordinate.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_theta(omega: f64, tau: f64) -> f64 {
    (omega * tau).atan()
}
