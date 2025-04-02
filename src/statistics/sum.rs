/// Compute the sum of the input array.
///
/// # Arguments
///
/// * `input` - A slice of numbers.
///
/// # Examples
///
/// ```
/// use imgal::math::sum;
///
/// let int_data = [2, 5, 10, 18];
/// assert_eq!(sum(&int_data), 35);
///
/// let float_data = [1.82, 3.35, 7.13, 9.25];
/// assert_eq!(sum(&float_data), 21.55);
/// ```
pub fn sum<T: std::ops::Add<Output = T> + Copy + Default>(input: &[T]) -> T {
    let mut output = T::default();
    for &v in input {
        output = output + v;
    }
    output
}
