use ndarray::Axis;

use imgal_core::parameters::omega;
use imgal_core::phasor::calibration;
use imgal_core::phasor::plot;
use imgal_core::phasor::time_domain;
use imgal_core::simulation::decay;

// test the phasor::calibration
#[test]
fn calibration_coordinate_pair() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(1.25e-8);
    let (g, s) = plot::single_component_coordinate_pair(1.1e-9, w);

    // set a modulation and phase value to calibrate with
    let modulation = 1.05;
    let phase = 0.42;
    let coords_cal = calibration::coordinate_pair(g, s, modulation, phase);

    assert_eq!(coords_cal, (0.5529599928454205, 0.7338912847329425));
}

#[test]
fn calibration_image() {
    // set decay simulation parameters
    let samples = 256;
    let period = 1.25e-8;
    let tau = 1.1e-9;
    let initial_value = 120.0;
    let irf_width = 7.324e-10;
    let irf_center = 2.0e-9;
    let shape = (10, 10);

    // create simulated ideal decay data
    let sim_data = decay::gaussian_fluorescence_3d(
        samples,
        period,
        tau,
        initial_value,
        irf_width,
        irf_center,
        shape,
    );

    // calculate the phasor image, (G, S)
    let gs_arr = time_domain::image(&sim_data, period, None, None);

    // calibrate the phasor image
    let modulation = 1.05;
    let phase = -0.981;
    let cal_gs_arr = calibration::image(&gs_arr, modulation, phase, None);

    // pick a point in the calibrated data
    let g_mean = cal_gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = cal_gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert_eq!(g_mean, 0.7923112328362736);
    assert_eq!(s_mean, 0.4449453208898232);
}

#[test]
fn calibration_image_mut() {
    // set decay simulation parameters
    let samples = 256;
    let period = 1.25e-8;
    let tau = 1.1e-9;
    let initial_value = 120.0;
    let irf_width = 7.324e-10;
    let irf_center = 2.0e-9;
    let shape = (10, 10);

    // create simulated ideal decay data
    let sim_data = decay::gaussian_fluorescence_3d(
        samples,
        period,
        tau,
        initial_value,
        irf_width,
        irf_center,
        shape,
    );

    // calculate the phasor image, (G, S)
    let mut gs_arr = time_domain::image(&sim_data, period, None, None);

    // calibrate the phasor image
    let modulation = 1.05;
    let phase = -0.981;
    calibration::image_mut(gs_arr.view_mut(), modulation, phase, None);

    // pick a point in the calibrated data
    let g_mean = gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert_eq!(g_mean, 0.7923112328362734);
    assert_eq!(s_mean, 0.444945320889823);
}

#[test]
fn calibration_modulation_and_phase() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(1.25e-8);
    let mod_phs = calibration::modulation_and_phase(-0.055, 0.59, 1.1e-9, w);

    assert_eq!(mod_phs, (1.4768757234403935, -1.1586655116823268));
}

// test the phasor::plot module
#[test]
fn plot_modulation() {
    let m = plot::modulation(0.71, 0.43);

    assert_eq!(m, 0.8300602387778853);
}

#[test]
fn plot_phase() {
    let p = plot::phase(0.71, 0.43);

    assert_eq!(p, 0.5445517081560367);
}

#[test]
fn plot_single_component_coordinate_pair() {
    // use 1.1 ns tau and 12.5 ns period
    let w = omega(1.25e-8);
    let coords = plot::single_component_coordinate_pair(1.1e-9, w);

    assert_eq!(coords, (0.7658604730109535, 0.4234598078807387));
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
