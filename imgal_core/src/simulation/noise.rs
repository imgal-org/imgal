use ndarray::{Array1, Array3, ArrayView1, ArrayView3, ArrayViewMut1, ArrayViewMut3, Axis, Zip};
use rand::SeedableRng;
use rand::prelude::*;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Poisson};
use rayon::prelude::*;

use crate::traits::numeric::ToFloat64;

/// Simulate Poisson noise on a 1-dimensional array.
///
/// # Description
///
/// This function applies Poisson noise (_i.e._ shot noise) on a 1-dimensional
/// array of data. An element-wise lambda value (scaled by the `scale` parameter)
/// is used to simulate the Poisson noise with variable signal strength.
///
/// This function creates a new array and does not mutate the input array.
///
/// # Arguments
///
/// * `data`: The input 1-dimensional array.
/// * `scale`: The scale factor.
/// * `seed`: Pseudorandom number generator seed. Set the `seed` value to apply
///    homogenous noise to the input array. If `None`, then heterogenous noise
///    is applied to the input array.
///
/// # Returns
///
/// * `Array1<f64>`: A 1-dimensonal array of the input data with Poisson noise applied.
pub fn poisson_1d<T>(data: ArrayView1<T>, scale: f64, seed: Option<u64>) -> Array1<f64>
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let s = seed.unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(s);

    // create new array and apply noise
    let n_data: Array1<f64> = data.map(|x| {
        if (*x).into() > 0.0 {
            let l: f64 = (*x).into() * scale;
            let p = Poisson::new(l).unwrap();
            p.sample(&mut rng)
        } else {
            0.0
        }
    });

    n_data
}

/// Simulate Poisson noise on a 1-dimensional array.
///
/// # Description
///
/// This function applies Poisson noise (_i.e._ shot noise) on a 1-dimensional
/// array of data. An element-wise lambda value (scaled by the `scale` parameter)
/// is used to simulate the Poisson noise with variable signal strength.
///
/// This function mutates the input array and does not create a new array.
///
/// # Arguments
///
/// * `data`: The input 1-dimensional array to mutate.
/// * `scale`: The scale factor.
/// * `seed`: Pseudorandom number generator seed. Set the `seed` value to apply
///    homogenous noise to the input array. If `None`, then heterogenous noise
///    is applied to the input array.
pub fn poisson_1d_mut(mut data: ArrayViewMut1<f64>, scale: f64, seed: Option<u64>) {
    // set optional parameters if needed
    let s = seed.unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(s);

    // mutate the 1d data array
    data.map_inplace(|x| {
        if *x > 0.0 {
            let l = *x * scale;
            let p = Poisson::new(l).unwrap();
            *x = p.sample(&mut rng);
        } else {
            *x = 0.0;
        }
    });
}

/// Simulate Poisson noise on a 3-dimensional array.
///
/// # Description
///
/// This function applies Poisson noise (_i.e._ shot noise) on a 3-dimensional
/// array of data. An element-wise lambda value (scaled by the `scale` parameter)
/// is used to simulate Poisson noise with variable signal strength.
///
/// This function creates a new array and does not mutate the input array.
///
/// # Arguments
///
/// * `data`: The input 3-dimensional array.
/// * `scale`: The scale factor.
/// * `seed`: Pseudorandom number generator seed. Set the `seed` value to apply
///    homogenous noise to the input array. If `None`, then heterogenous noise
///    is applied to the input array.
/// * `axis`: The signal data axis, default = 2.
///
/// # Returns
///
/// * `Array3<f64>`: A 3-dimensional array of the input data with Poisson noise
///    applied.
pub fn poisson_3d<T>(
    data: ArrayView3<T>,
    scale: f64,
    seed: Option<u64>,
    axis: Option<usize>,
) -> Array3<f64>
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let a = axis.unwrap_or(2);

    // allocate new array of same shape for noise data
    let shape = data.dim();
    let mut n_data = Array3::<f64>::zeros(shape);

    // apply and store Poisson noise data in new array
    let src_lanes = data.lanes(Axis(a));
    let dst_lanes = n_data.lanes_mut(Axis(a));
    if let Some(s) = seed {
        // apply noise with one seed, homogenous noise
        Zip::from(src_lanes)
            .and(dst_lanes)
            .par_for_each(|s_ln, d_ln| {
                let mut rng = StdRng::seed_from_u64(s);
                Zip::from(s_ln).and(d_ln).for_each(|s, d| {
                    if (*s).into() > 0.0 {
                        let l = (*s).into() * scale;
                        let p = Poisson::new(l).unwrap();
                        *d = p.sample(&mut rng);
                    } else {
                        *d = 0.0;
                    }
                });
            });
    } else {
        // apply noise with variable seeds, hetergenous noise
        Zip::from(src_lanes)
            .and(dst_lanes)
            .par_for_each(|s_ln, d_ln| {
                let mut rng = rand::rng();
                Zip::from(s_ln).and(d_ln).for_each(|s, d| {
                    if (*s).into() > 0.0 {
                        let l = (*s).into() * scale;
                        let p = Poisson::new(l).unwrap();
                        *d = p.sample(&mut rng);
                    } else {
                        *d = 0.0
                    }
                });
            });
    }

    n_data
}

/// Simulate Poisson noise on a 3-dimensional array.
///
/// # Description
///
/// This function applies Poisson noise (_i.e._ shot noise) on a 3-dimensional
/// array of data. An element-wise lambda value (scaled by the `scale` parameter)
/// is used to simulate Poisson noise with variable signal strength.
///
/// This function mutates the input array and does not create a new array.
///
/// # Arguments
///
/// * `data`: The input 3-dimensional array to mutate.
/// * `scale`: The scale factor.
/// * `seed`: Pseudorandom number generator seed. Set the `seed` value to apply
///    homogenous noise to the input array. If `None`, then heterogenous noise
///    is applied to the input array.
/// * `axis`: The signal data axis, default = 2.
pub fn poisson_3d_mut(
    mut data: ArrayViewMut3<f64>,
    scale: f64,
    seed: Option<u64>,
    axis: Option<usize>,
) {
    // set optional parameters if needed
    let a = axis.unwrap_or(2);

    // apply noise to each lane
    let lanes = data.lanes_mut(Axis(a));
    if let Some(s) = seed {
        // apply noise with one seed, homogeneous noise
        lanes.into_iter().par_bridge().for_each(|ln| {
            poisson_1d_mut(ln, scale, Some(s));
        });
    } else {
        // apply noise with variable seeds, hetergeneous noise
        lanes.into_iter().par_bridge().for_each(|ln| {
            let mut rng = rand::rng();
            let s = rng.next_u64();
            poisson_1d_mut(ln, scale, Some(s));
        });
    }
}
