use std::f64;

/// Compute the modulation of a multi-component phasor coordinate pair.
///
/// # Description
///
/// The modulation of a multi-component (_i.e._ inside the universal circle) phasor
/// coordinate pair is calculated using:
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
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the phi polar angle of a multi-component phasor coordinate pair.
///
/// # Description
///
/// The phi (φ) of a multi-component (_i.e._ inside the universal circle) phasor
/// coordinate pair is calculated using:
///
/// φ = tan⁻¹(S / G)
///
/// Computes atan(S/G) in all four quadrants using atan2.
///
/// # Arguments
///
/// * `g`: The real component, G.
/// * `s`: The imaginary component, S.
///
/// # Returns
///
/// * `f64`: The phi (φ) angle of the (G, S) phasor coordinate pair.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn multi_component_phi(g: f64, s: f64) -> f64 {
    s.atan2(g)
}

/// Compute the modulation of a single-component phasor coordinate pair.
///
/// # Description
///
/// The modulation (M) of a single-component (_i.e._ on the universal circle) phasor
/// coordinate pair is calculated using:
///
/// M = cos(φ)
///
/// # Arguments
///
/// * `phi`: The phi (φ) angle of the (G, S) phasor coordinate pair.
///
/// # Returns
///
/// * `f64`: The modulation (M) of the (G, S) phasor coordiate pair.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_modulation(phi: f64) -> f64 {
    f64::cos(phi)
}

/// Compute the phi angle of a single-component phasor coordinate pair.
///
/// # Description
///
/// The phi (φ) angle of a single-component (_i.e_ on the universal circle) phasor
/// coordinate pair is calculated using:
///
///  φ = tan⁻¹(ω * τ)
///
/// # Arguments
///
/// * `omega`: The omega (ω), angular frequency.
/// * `tau`: The tau (τ), lifetime.
///
/// # Returns
///
/// * `f64`: The phi (φ) angle of the (G, S) phasor coordinate pair.
///
/// # Reference
///
/// <https://doi.org/10.1146/annurev-biophys-062920-063631>
pub fn single_component_phi(omega: f64, tau: f64) -> f64 {
    (omega * tau).atan()
}
