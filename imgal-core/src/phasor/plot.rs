use std::f64;

/// Compute the modulation of a phasor point.
///
/// # Description
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
pub fn modulation(g: f64, s: f64) -> f64 {
    let g_sqr: f64 = g * g;
    let s_sqr: f64 = s * s;
    f64::sqrt(g_sqr + s_sqr)
}

/// Compute the theta of a phsor point.
///
/// # Description
///
/// theta = tan^-1 * (S / G)
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
pub fn theta(g: f64, s: f64) -> f64 {
    s.atan2(g)
}
