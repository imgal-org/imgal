use imgal_core::distribution;
use imgal_core::integration::midpoint;

#[test]
fn distribution_gaussian() {
    // create a gaussian distribution
    let gauss_arr = distribution::gaussian(2.0, 256, 4.0, 2.0);

    // assert a value and integrate the curve
    assert_eq!(gauss_arr[100], 0.004465507286912305);
    assert_eq!(midpoint(gauss_arr.view(), None), 1.0000000000000007);
}
