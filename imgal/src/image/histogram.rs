use ndarray::ArrayViewD;

use crate::statistics::min_max;
use crate::traits::numeric::ToFloat64;

/// TODO histogram doc
pub fn histogram<T>(data: ArrayViewD<T>, bins: Option<usize>) -> Vec<i64>
where
    T: ToFloat64,
{
    // TODO: set reasonable default bins
    // TODO: parallelize this
    let bins = bins.unwrap_or(256);

    // return an empty histogram if bins is zero or array is zero
    if data.is_empty() || bins == 0 {
        return vec![0; 1];
    }

    // get min and max values
    let (min, max) = min_max(data.view());

    // construct histogram
    let mut hist = vec![0; bins];
    let bin_width: f64 = (max.into() - min.into()) / bins as f64;

    // populate histogram
    data.iter().for_each(|&v| {
        let bin_index: usize = ((v.into() - min.into()) / bin_width) as usize;
        let bin_index = bin_index.min(bins - 1);
        hist[bin_index] += 1;
    });

    hist
}
