use ndarray::ArrayViewD;

use crate::statistics::min_max;
use crate::traits::numeric::ToFloat64;

/// Compute the image histogram from an n-dimensional array.
///
/// # Description
///
/// This function computes an image (_i.e._ frequency) histogram for the values
/// in the input n-dimensional array.
///
/// # Arguments
///
/// * `data`: The input n-dimensional array to construct the histogram from.
/// * `bins`: The number of bins to use for the histogram, default = 256.
///
/// # Returns
///
/// * `Vec<i64>`: The histogram of the input n-dimensional array of size `bins`.
///    Each element represents the count of values falling into the
///    corresponding bin.
pub fn histogram<T>(data: ArrayViewD<T>, bins: Option<usize>) -> Vec<i64>
where
    T: ToFloat64,
{
    let bins = bins.unwrap_or(256);

    // return an empty histogram if bins is zero or array is zero
    if data.is_empty() || bins == 0 {
        return vec![0; 1];
    }

    // get min and max values
    let (min, max) = min_max(data.view());

    // construct histogram
    let mut hist = vec![0; bins];
    let bin_width: f64 = (max.to_f64() - min.to_f64()) / bins as f64;
    data.iter().for_each(|&v| {
        let bin_index: usize = ((v.to_f64() - min.to_f64()) / bin_width) as usize;
        let bin_index = bin_index.min(bins - 1);
        hist[bin_index] += 1;
    });

    hist
}
