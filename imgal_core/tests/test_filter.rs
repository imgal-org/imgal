use imgal_core::filter;
use imgal_core::integration::midpoint;
use imgal_core::simulation::{decay, instrument};

#[test]
fn filter_fft_convolve_1d() {
    let a = decay::ideal_fluorescence_1d(256, 1.25e-8, 1.1e-9, 100.0);
    let b = instrument::gaussian_irf_1d(256, 1.25e-8, 1.0e-9, 2.0e-9);
    let conv = filter::fft_convolve_1d(a.view(), b.view());

    // check curve by integration and the peak of the curve
    assert_eq!(midpoint(conv.view(), None), 2294.2022514983837);
    assert_eq!(conv[50], 54.95117763418145);
}

#[test]
fn filter_fft_deconvolve_1d() {
    let a = decay::gaussian_fluorescence_1d(256, 1.25e-8, 1.1e-9, 100.0, 1.0e-9, 2.0e-9);
    let b = decay::ideal_fluorescence_1d(256, 1.25e-8, 1.1e-9, 100.0);
    let dconv = filter::fft_deconvolve_1d(a.view(), b.view(), None);

    // check curve by integration and the peak of the curve
    assert_eq!(midpoint(dconv.view(), None), 0.9999999993049233);
    assert_eq!(dconv[40], 0.045854951924008645);
}
