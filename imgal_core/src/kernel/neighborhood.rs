use ndarray::{Array2, Array3};

use crate::error::ArrayError;

/// Create a 2-dimensional square kernel with a circle neighborhood.
///
/// # Description
///
/// This function creates a square boolean kernel representing a filled circle
/// of the specified radius (_i.e._ the neighborhood). The circle is defined
/// using the Euclidean distance from the center point. Points within the
/// radius are set to `true`, while points outside are set to `false`.
///
/// # Arguments
///
/// * `radius`: The radius of the circle in pixels. Must be greather than 0.
///
/// # Returns
///
/// * `Ok(Array2<bool>)`: A 2-dimensional square boolean array with side lengths
///    of `radius * 2 + 1` where `true` values represent points inside or on the
///    circle boundary of the specified radius.
/// * `Err(ArrayError)`: An ArrayError.
pub fn circle(radius: usize) -> Result<Array2<bool>, ArrayError> {
    // check if radius parameter is valid
    if radius == 0 {
        return Err(ArrayError::InvalidArrayParameterValueLess {
            param_name: "radius",
            value: 0,
        });
    }

    // set circle parameters and create kernel
    let dim = radius * 2 + 1;
    let center = radius as f64;
    let mut kernel = Array2::<bool>::default((dim, dim));

    // iterate through each position and calculate euclidean distance
    kernel.indexed_iter_mut().for_each(|((row, col), v)| {
        let x = col as f64;
        let y = row as f64;
        let dist = ((x - center).powi(2) + (y - center).powi(2)).sqrt();
        *v = dist <= center;
    });

    Ok(kernel)
}

/// Create a 3-dimensional cube kernel with a sphere neighborhood.
///
/// # Description
///
/// This function creates a cube boolean kernel representing a filled sphere of
/// the specified radius (_i.e_ the neighborhood). The sphere is defined using
/// the Euclidean distance from the center point. Points within the radius are
/// set to `true`, while jpoints outside are set to `false`.
///
/// # Arguments
///
/// * `radius`: The radius of the sphere in voxels. Must be greater than 0.
///
/// # Returns
///
/// * `Ok(Array3<bool>)`: A 3-dimensional cube boolean array with side lengths
///   of `radius * 2 + 1` where `true` values represent points inside or on the
///   sphere boundary of the specified radius.
/// * `Err(ArrayError)`: An ArrayError.
pub fn sphere(radius: usize) -> Result<Array3<bool>, ArrayError> {
    // check if radius parameter is valid
    if radius == 0 {
        return Err(ArrayError::InvalidArrayParameterValueEqual {
            param_name: "radius",
            value: 0,
        });
    }

    // set sphere parameters and create kernel
    let dim = radius * 2 + 1;
    let center = radius as f64;
    let mut kernel = Array3::<bool>::default((dim, dim, dim));

    // iterate through each position and caluclate euclidean distance
    kernel.indexed_iter_mut().for_each(|((pln, row, col), v)| {
        let x = col as f64;
        let y = row as f64;
        let z = pln as f64;
        let dist = ((x - center).powi(2) + (y - center).powi(2) + (z - center).powi(2)).sqrt();
        *v = dist <= center;
    });

    Ok(kernel)
}
