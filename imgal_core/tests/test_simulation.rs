use ndarray::s;

use imgal_core::integration::midpoint;
use imgal_core::simulation::{decay, instrument};

// simulated bioexponential decay parameters
const SAMPLES: usize = 256;
const PERIOD: f64 = 12.5;
const TAUS: [f64; 2] = [1.0, 3.0];
const FRACTIONS: [f64; 2] = [0.7, 0.3];
const TOTAL_COUNTS: f64 = 5000.0;
const IRF_CENTER: f64 = 3.0;
const IRF_WIDTH: f64 = 0.5;
const SHAPE: (usize, usize) = (10, 10);

// helper functions
fn ensure_within_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

#[test]
fn decay_gaussian_exponential_1d() {
    // simulate decay data
    let i = decay::gaussian_exponential_1d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
    )
    .unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // integrate the curve for photon count (approx) and check peak curve value
    assert!(ensure_within_tolerance(
        midpoint(&i, Some(dt)),
        5015.983504781878,
        1e-12
    ));
    assert!(ensure_within_tolerance(i[68], 2810.4960313074985, 1e-12));
}

#[test]
fn decay_gaussian_exponential_3d() {
    // simulate decay data
    let i = decay::gaussian_exponential_3d(
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        IRF_CENTER,
        IRF_WIDTH,
        SHAPE,
    )
    .unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check shape, curve by integration and a point
    assert_eq!(i.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(i.slice(s![5, 5, ..]).as_slice().unwrap(), Some(dt)),
        5015.983504781878,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        i[[5, 5, 68]],
        2810.4960313074985,
        1e-12
    ));
}

#[test]
fn decay_ideal_exponential_1d() {
    // simulate decay data
    let i = decay::ideal_exponential_1d(SAMPLES, PERIOD, &TAUS, &FRACTIONS, TOTAL_COUNTS).unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(&i, Some(dt)),
        5055.86745659704,
        1e-12
    ));
    assert!(ensure_within_tolerance(i[30], 1110.5191029245611, 1e-12));
}

#[test]
fn decay_ideal_exponential_3d() {
    // simulate decay data
    let i = decay::ideal_exponential_3d(SAMPLES, PERIOD, &TAUS, &FRACTIONS, TOTAL_COUNTS, SHAPE)
        .unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check shape, curve by integration and a point
    assert_eq!(i.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(i.slice(s![5, 5, ..]).as_slice().unwrap(), Some(dt)),
        5055.86745659704,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        i[[5, 5, 30]],
        1110.5191029245611,
        1e-12
    ));
}

#[test]
fn decay_irf_exponential_1d() {
    // simulate IRF data to convolve decay data
    let irf = instrument::gaussian_irf_1d(SAMPLES, PERIOD, IRF_CENTER, IRF_WIDTH);
    let i =
        decay::irf_exponential_1d(&irf, SAMPLES, PERIOD, &TAUS, &FRACTIONS, TOTAL_COUNTS).unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(&i, Some(dt)),
        5015.983504781878,
        1e-12
    ));
    assert!(ensure_within_tolerance(i[68], 2810.4960313074985, 1e-12));
}

#[test]
fn decay_irf_exponential_3d() {
    // simulate IRF data to convolve decay data
    let irf = instrument::gaussian_irf_1d(SAMPLES, PERIOD, IRF_CENTER, IRF_WIDTH);
    let i = decay::irf_exponential_3d(
        &irf,
        SAMPLES,
        PERIOD,
        &TAUS,
        &FRACTIONS,
        TOTAL_COUNTS,
        SHAPE,
    )
    .unwrap();

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check shape, cruve by integration and a point
    assert_eq!(i.shape(), [10, 10, 256]);
    assert!(ensure_within_tolerance(
        midpoint(i.slice(s![5, 5, ..]).as_slice().unwrap(), Some(dt)),
        5015.983504781878,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        i[[5, 5, 68]],
        2810.4960313074985,
        1e-12
    ));
}

#[test]
fn instrument_gaussian_irf_1d() {
    // simulate IRF data
    let irf = instrument::gaussian_irf_1d(SAMPLES, PERIOD, IRF_CENTER, IRF_WIDTH);

    // bin width for integration check
    let dt = PERIOD / SAMPLES as f64;

    // check the curve by integration and a point
    assert!(ensure_within_tolerance(
        midpoint(&irf, Some(dt)),
        0.048828125,
        1e-12
    ));
    assert!(ensure_within_tolerance(irf[62], 0.09054417121965984, 1e-12));
}
