use ndarray::Array1;

use imgal_core::distribution::gaussian;
use imgal_core::integration;

// helper functions
fn get_gaussian_distribution(bins: usize) -> Array1<f64> {
    gaussian(2.0, bins, 4.0, 2.0)
}

#[test]
fn integration_composite_simpson() {
    let gauss_arr = get_gaussian_distribution(512);

    assert_eq!(
        integration::composite_simpson(gauss_arr.view(), None),
        0.9986155934120933
    );
}

#[test]
fn integration_midpoint() {
    let gauss_arr = get_gaussian_distribution(512);

    assert_eq!(integration::midpoint(gauss_arr.view(), None), 1.0000000000000009);
}

#[test]
fn integration_simpson() {
    let gauss_arr = get_gaussian_distribution(511);

    assert_eq!(
        integration::simpson(gauss_arr.view(), None).unwrap(),
        0.9986128844345734
    );
}
