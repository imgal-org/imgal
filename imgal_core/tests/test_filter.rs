use imgal_core::filter;
use imgal_core::integration::midpoint;
use imgal_core::simulation::{decay, instrument};

// helper functions
fn ensure_within_tolerance(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() < tolerance
}

#[test]
fn filter_fft_convolve_1d() {
    let a = decay::ideal_monoexponential_1d(256, 1.25e-8, 1.1e-9, 100.0);
    let b = instrument::gaussian_irf_1d(256, 1.25e-8, 1.0e-9, 2.0e-9);
    let conv = filter::fft_convolve_1d(a.view(), b.view());

    // check curve by integration and the peak of the curve
    assert!(ensure_within_tolerance(
        midpoint(conv.view(), None),
        2294.2022514983837,
        1e-12
    ));
    assert!(ensure_within_tolerance(conv[50], 54.95117763418145, 1e-12));
}

#[test]
fn filter_fft_deconvolve_1d() {
    let a = decay::gaussian_monoexponential_1d(256, 1.25e-8, 1.1e-9, 100.0, 1.0e-9, 2.0e-9);
    let b = decay::ideal_monoexponential_1d(256, 1.25e-8, 1.1e-9, 100.0);
    let dconv = filter::fft_deconvolve_1d(a.view(), b.view(), None);

    // check curve by integration and the peak of the curve
    assert!(ensure_within_tolerance(
        midpoint(dconv.view(), None),
        0.999999999304922,
        1e-12
    ));
    assert!(ensure_within_tolerance(
        dconv[40],
        0.045854951924008645,
        1e-12
    ));
}
