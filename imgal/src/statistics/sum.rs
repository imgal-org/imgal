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
/// use imgal::statistics::sum;
///
/// // create a 1-dimensional array
/// let arr = [1.82, 3.35, 7.13, 9.25];
///
/// // compute the sum of the array
/// let total = sum(&arr);
///
/// assert_eq!(total, 21.55);
/// ```
pub fn sum<T>(data: &[T]) -> T
where
    T: ToFloat64,
{
    data.iter().copied().sum()
}
