use std::iter::Sum;

/// Compute the sum of the slice of numbers.
///
/// # Description
///
/// Computes the sum of numbers in the input slice.
///
/// # Arguments
///
/// * `input`: A slice of numbers.
///
/// # Returns
///
/// * `f64`: The sum.
///
/// # Examples
///
/// ```
/// use imgal_core::statistics::sum;
///
/// let int_data = [2, 5, 10, 18];
/// assert_eq!(sum(&int_data), 35);
///
/// let float_data = [1.82, 3.35, 7.13, 9.25];
/// assert_eq!(sum(&float_data), 21.55);
/// ```
pub fn sum<T: Copy + Sum>(input: &[T]) -> T {
    input.iter().copied().sum()
}
