use ndarray::s;

use imgal_core::integration::midpoint;
use imgal_core::simulation::{decay, instrument, noise};

// helper functions
fn ensure_within_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

#[test]
fn decay_gaussian_fluorescence_1d() {
    let d = decay::gaussian_fluorescence_1d(256, 1.25e-8, 1.0e-9, 100.0, 0.5e-9, 3.0e-9);

    // check curve by integration and the peak of the curve
    assert!(ensure_within_tolerance(
        midpoint(d.view(), None),
        2090.2560980644535,
        1e-12
    ));
    assert!(ensure_within_tolerance(d[68], 68.16128303586186, 1e-12));
}

#[test]
fn decay_gaussian_fluorescence_3d() {
    let d = decay::gaussian_fluorescence_3d(256, 1.25e-8, 1.0e-9, 100.0, 0.5e-9, 3.0e-9, (10, 10));

    // check shape, curve by integration and a point
    assert_eq!(d.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(d.slice(s![5, 5, ..]), None),
        2090.2560980644535,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        d[[5, 5, 68]],
        68.16128303586186,
        1e-12
    ));
}

#[test]
fn decay_ideal_fluorescence_1d() {
    let d = decay::ideal_fluorescence_1d(256, 1.25e-8, 1.0e-9, 100.0);

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(d.view(), None),
        2090.401062811126,
        1e-12
    ));
    assert!(ensure_within_tolerance(d[30], 22.979027467301684, 1e-12));
}

#[test]
fn decay_ideal_fluorescence_3d() {
    let d = decay::ideal_fluorescence_3d(256, 1.25e-8, 1.0e-9, 100.0, (10, 10));

    // check shape, curve by integration and a point
    assert_eq!(d.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(d.slice(s![5, 5, ..]), None),
        2090.401062811126,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        d[[5, 5, 30]],
        22.979027467301684,
        1e-12
    ));
}

#[test]
fn decay_irf_fluorescence_1d() {
    let irf = instrument::gaussian_irf_1d(256, 1.25e-8, 0.5e-9, 3.0e-9);
    let d = decay::irf_fluorescence_1d(irf.view(), 256, 1.25e-8, 1.0e-9, 100.0);

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(d.view(), None),
        2090.2560980644535,
        1e-12
    ));
    assert!(ensure_within_tolerance(d[68], 68.16128303586186, 1e-12));
}

#[test]
fn decay_irf_fluorescence_3d() {
    let irf = instrument::gaussian_irf_1d(256, 1.25e-8, 0.5e-9, 3.0e-9);
    let d = decay::irf_fluorescence_3d(irf.view(), 256, 1.25e-8, 1.0e-9, 100.0, (10, 10));

    // check shape, cruve by integration and a point
    assert_eq!(d.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(d.slice(s![5, 5, ..]), None),
        2090.2560980644535,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        d[[5, 5, 68]],
        68.16128303586186,
        1e-12
    ));
}

#[test]
fn instrument_gaussian_irf_1d() {
    let irf = instrument::gaussian_irf_1d(256, 1.25e-8, 0.5e-9, 3.0e-9);

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(irf.view(), None),
        0.9999999999999999,
        1e-12
    ));
    assert!(ensure_within_tolerance(irf[62], 0.09054417121965984, 1e-12));
}
