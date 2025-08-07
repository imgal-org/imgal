use ndarray::{Array2, Array3, Axis};

use imgal_core::parameter::omega;
use imgal_core::phasor::calibration;
use imgal_core::phasor::plot;
use imgal_core::phasor::time_domain;
use imgal_core::simulation::decay;

// helper functions
fn ensure_within_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

fn get_decay_data(shape: (usize, usize)) -> Array3<f64> {
    // set decay simulation parameters
    let samples = 256;
    let period = 1.25e-8;
    let tau = 1.1e-9;
    let initial_value = 120.0;
    let irf_width = 7.324e-10;
    let irf_center = 2.0e-9;

    // create simulated ideal decay data
    decay::gaussian_fluorescence_3d(
        samples,
        period,
        tau,
        initial_value,
        irf_width,
        irf_center,
        shape,
    )
}

fn get_circle_mask(shape: (usize, usize), center: (isize, isize), radius: isize) -> Array2<bool> {
    // set circle parameters
    let (row, col) = shape;
    let (cx, cy) = center;
    let r2 = radius * radius;
    let y_min = (cy - radius).max(0);
    let y_max = (cy + radius).min(row as isize - 1);
    let x_min = (cx - radius).max(0);
    let x_max = (cx + radius).min(col as isize - 1);

    // create empty bool array and a filled draw circle
    let mut mask = Array2::<bool>::default(shape);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let dx = cx - x;
            let dy = cy - y;
            // use the squared distance formula for a quick circle mask
            if dx * dx + dy * dy <= r2 {
                mask[[y as usize, x as usize]] = true;
            }
        }
    }

    mask
}

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
    // get simulated data
    let sim_data = get_decay_data((10, 10));

    // calculate the phasor image, (G, S)
    let gs_arr = time_domain::image(&sim_data, 1.25e-8, None, None, None).unwrap();

    // calibrate the phasor image
    let modulation = 1.05;
    let phase = -0.981;
    let cal_gs_arr = calibration::image(&gs_arr, modulation, phase, None);

    // pick a point in the calibrated data
    let g_mean = cal_gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = cal_gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert!(ensure_within_tolerance(g_mean, 0.79231123283627, 1e-12));
    assert!(ensure_within_tolerance(s_mean, 0.44494532088982, 1e-12));
}

#[test]
fn calibration_image_mut() {
    // get simulated data
    let sim_data = get_decay_data((10, 10));

    // calculate the phasor image, (G, S)
    let mut gs_arr = time_domain::image(&sim_data, 1.25e-8, None, None, None).unwrap();

    // calibrate the phasor image
    let modulation = 1.05;
    let phase = -0.981;
    calibration::image_mut(gs_arr.view_mut(), modulation, phase, None);

    // pick a point in the calibrated data
    let g_mean = gs_arr.index_axis(Axis(2), 0).mean().unwrap();
    let s_mean = gs_arr.index_axis(Axis(2), 1).mean().unwrap();

    assert!(ensure_within_tolerance(g_mean, 0.79231123283627, 1e-12));
    assert!(ensure_within_tolerance(s_mean, 0.44494532088982, 1e-12));
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
fn time_domain_image() {
    // get simulated data and circle mask
    let sim_data = get_decay_data((100, 100));
    let mask = get_circle_mask((100, 100), (50, 50), 8);

    // compute phasors with and without a mask
    let gs_no_mask = time_domain::image(&sim_data, 1.25e-8, None, None, None).unwrap();
    let gs_with_mask =
        time_domain::image(&sim_data, 1.25e-8, Some(mask.view()), None, None).unwrap();

    // get views of each channel
    let g_no_mask_view = gs_no_mask.index_axis(Axis(2), 0);
    let s_no_mask_view = gs_no_mask.index_axis(Axis(2), 1);
    let g_with_mask_view = gs_with_mask.index_axis(Axis(2), 0);
    let s_with_mask_view = gs_with_mask.index_axis(Axis(2), 1);

    // expected values
    let exp_g = 0.067527056199306;
    let exp_s = 0.862788883482716;

    // assert G and S values, no mask
    assert!(ensure_within_tolerance(
        g_no_mask_view.mean().unwrap(),
        exp_g,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_no_mask_view.mean().unwrap(),
        exp_s,
        1e-12
    ));

    // assert G, S and 0.0 values, with mask
    assert!(ensure_within_tolerance(
        g_with_mask_view[[45, 52]],
        exp_g,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_with_mask_view[[45, 52]],
        exp_s,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        g_with_mask_view[[5, 8]],
        0.0,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        s_with_mask_view[[5, 8]],
        0.0,
        1e-12
    ));
}

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
