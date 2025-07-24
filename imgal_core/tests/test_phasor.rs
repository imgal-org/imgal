use imgal_core::simulation::decay;
use imgal_core::phasor::time_domain;

#[test]
fn test_time_domain_imaginary() {
    let data = decay::ideal_fluorescence_1d(256, 1.25e-8, 4.0e-9, 100.0);
    let s = time_domain::imaginary(&data, 1.25e-8, None, None);
    assert_eq!(s, 0.39720439791434226);
}

#[test]
fn test_time_domain_real() {
    let data = decay::ideal_fluorescence_1d(256, 1.25e-8, 4.0e-9, 100.0);
    let g = time_domain::real(&data, 1.25e-8, None, None);
    assert_eq!(g, 0.20444291541716833);
}
