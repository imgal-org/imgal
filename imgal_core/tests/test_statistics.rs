use ndarray::Array1;

use imgal_core::statistics;

#[test]
fn statistics_sum() {
    // create some test vecs
    let int_data = vec![2, 5, 10, 23];
    let float_data = vec![1.0, 10.5, 3.25, 37.11];

    // convert vecs into Array1
    let int_arr = Array1::from_vec(int_data);
    let float_arr = Array1::from_vec(float_data);

    // assert arrays
    assert_eq!(statistics::sum(int_arr.view()), 40);
    assert_eq!(statistics::sum(float_arr.view()), 51.86);
}
