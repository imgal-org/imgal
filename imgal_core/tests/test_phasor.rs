// use imgal_core::phasor::calibration;
use imgal_core::parameters::omega;
use imgal_core::phasor::plot;
use imgal_core::phasor::time_domain;
use imgal_core::simulation::decay;

// test the phasor::calibration
// test the phasor::plot module
#[test]
fn plot_modulation() {
    let m = plot::modulation(0.71, 0.43);
    assert_eq!(m, 0.8300602387778853)
}

#[test]
fn plot_phase() {
    let p = plot::phase(0.71, 0.43);
    assert_eq!(p, 0.5445517081560367)
}

#[test]
fn plot_single_component_coordinate_pair() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(1.25e-8);
    let coords = plot::single_component_coordinate_pair(1.1e-9, w);
    assert_eq!(coords, (0.7658604730109535, 0.4234598078807387))
}

// test the phasor::time_domain module
#[test]
fn time_domain_imaginary() {
    let data = decay::ideal_fluorescence_1d(256, 1.25e-8, 4.0e-9, 100.0);
    let s = time_domain::imaginary(&data, 1.25e-8, None, None);
    assert_eq!(s, 0.39720439791434226);
}

#[test]
fn time_domain_real() {
    let data = decay::ideal_fluorescence_1d(256, 1.25e-8, 4.0e-9, 100.0);
    let g = time_domain::real(&data, 1.25e-8, None, None);
    assert_eq!(g, 0.20444291541716833);
}
