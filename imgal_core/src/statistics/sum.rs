use ndarray::{ArrayBase, Data, Ix1};

use crate::traits::numeric::ToFloat64;

/// Compute the sum of the slice of numbers.
///
/// # Description
///
/// Computes the sum of numbers in the input slice.
///
/// # Arguments
///
/// * `data`: A slice of numbers.
///
/// # Returns
///
/// * `f64`: The sum.
///
/// # Examples
///
/// ```
/// use ndarray::Array1;
///
/// use imgal_core::statistics::sum;
///
/// // create a 1-dimensional array
/// let arr = Array1::from_vec(vec![1.82, 3.35, 7.13, 9.25]);
///
/// // compute the sum of the array
/// let total = sum(&arr);
///
/// assert_eq!(total, 21.55);
/// ```
pub fn sum<T, S>(data: &ArrayBase<S, Ix1>) -> T
where
    T: ToFloat64,
    S: Data<Elem = T>,
{
    let d = data.as_slice().unwrap();
    d.iter().copied().sum()
}
