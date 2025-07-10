use ndarray::Array1;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Distribution, Poisson};

use crate::traits::numeric::ToFloat64;

/// Simulate Poisson noise on a 1-dimensional array.
///
/// # Description
///
/// Apply Poisson noise (_i.e._ shot noise) on a 1-dimensional array of data. Here,
/// `data` typically represents signal data.
///
/// # Arguments
///
/// * `data`: The input 1-dimensonal array of signal values.
/// * `scale`: The scale factor.
/// * `seed`: Random number generator seed for reproducible results. If `None` the
///   default seed value is 0.
///
/// # Returns
///
/// * `Array1<f64>`: The 1-dimensonal array with Poisson noise applied.
pub fn poisson<T>(data: Array1<T>, scale: f64, seed: Option<u64>) -> Array1<f64>
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let s = seed.unwrap_or(0);
    let mut rng = StdRng::seed_from_u64(s);

    // apply noise
    let n_data: Array1<f64> = data.map(|x| {
        let lambda: f64 = (*x).into() * scale;
        let p = Poisson::new(lambda.max(0.0)).unwrap();
        p.sample(&mut rng)
    });
    n_data
}
