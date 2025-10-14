use ndarray::{Array, Array2};

use imgal::image;
use imgal::statistics::min_max;

#[test]
fn image_histogram() {
    // create data with known values and get the histogram
    let data = Array2::from_shape_fn((20, 20), |(i, j)| {
        if i < 15 {
            0
        } else {
            ((i - 15) * 20 + j) as u16
        }
    });
    let hist = image::histogram(data.view().into_dyn(), Some(20));

    // wrap hist vector as an array for assert tests
    let arr = Array::from_vec(hist);
    let mm = min_max(arr.view().into_dyn());

    assert_eq!(mm.0, 5);
    assert_eq!(mm.1, 305);
    assert_eq!(arr[0], 305);
    assert_eq!(arr[10], 5);
    assert_eq!(arr.len(), 20);
}
