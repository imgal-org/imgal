use ndarray::{Array2, Array3, ArrayView2, ArrayViewMut2, ArrayViewMut3, Axis, Zip};
use rayon::prelude::*;

use crate::error::ArrayError;
use crate::kernel::neighborhood::weighted_circle;
use crate::statistics::{effective_sample_size, weighted_kendall_tau_b};
use crate::traits::numeric::ToFloat64;

/// Compute colocalization strength using 2-dimensional Spatially Adaptive
/// Colocalization Analysis (SACA)
///
/// # Description
///
/// This function computes a pixel-wise _z-score_ indicating colocalization and
/// anti-colocalization strength on 2-dimensional input images using the
/// Spatially Adaptive Colocalization Analysis (SACA) framework. Per pixel SACA
/// utilizes a propagation and separation strategy to adaptively expand a
/// weighted circular kernel that defines the pixel of consideration's
/// neighborhood. The pixels within the neighborhood are assigned weights based
/// on their distance from the center pixel (decreasing with distance), ranked
/// and their colocalization coefficient computed using Kendall's Tau-b rank
/// correlation.
///
/// # Arguments
///
/// * `image_a`: The 2-dimensional input image, `A`. Image `A` must have the same
///    shape as image `B`.
/// * `image_b`: Ihe 2-dimensional input image, `B`. Image `B` must have the same
///    shape as image `A`.
/// * `threshold_a`: Pixel intensity threshold value for image `A`. Pixels below
///    this value are given a weight of 0.0 if the pixel is in the circular
///    neighborhood.
/// * `threshold_b`: Pixel intensity threshold value for image `B`. Pixels below
///    this value are given a weight of 0.0 if the pixel is in the circular
///    neighborhood.
///
/// # Returns
///
/// * `OK(Array2<f64>)`: The pixel-wise _z-score_ indicating colocalization or
///    anti-colocalization by its sign and the degree or strength of the
///    relationship through its absolute values.
/// * `Err(ArrayError)`: If the dimensions of image `A` and `B` do not match.
///
/// # Reference
///
/// <https://doi.org/10.1109/TIP.2019.2909194>
pub fn saca_2d<T>(
    image_a: ArrayView2<T>,
    image_b: ArrayView2<T>,
    threshold_a: T,
    threshold_b: T,
) -> Result<Array2<f64>, ArrayError>
where
    T: ToFloat64,
{
    // TODO make 2D output for now, final output should be 3D (heatmap + p-values)
    // ensure input images have the same shape
    let dims_a = image_a.dim();
    let dims_b = image_b.dim();
    if dims_a != dims_b {
        return Err(ArrayError::MismatchedArrayShapes {
            shape_a: vec![dims_a.0, dims_a.1],
            shape_b: vec![dims_b.0, dims_b.1],
        });
    }

    // create image buffers
    let mut result = Array2::<f64>::zeros(dims_a);
    let mut new_tau = Array2::<f64>::zeros(dims_a);
    let mut new_sqrt_n = Array2::<f64>::zeros(dims_a);
    let mut old_tau = Array2::<f64>::zeros(dims_a);
    let mut old_sqrt_n = Array2::<f64>::ones(dims_a);
    let mut stop = Array3::<f64>::zeros((dims_a.0, dims_a.1, 3));

    // set up saca parameters, see reference on "n" value selection for lambda
    let dn = ((dims_a.0 * dims_a.1) as f64).ln().sqrt() * 2.0;
    let lambda = dn * 1.0;
    let tu: usize = 15;
    let tl: usize = 8;
    let mut size_f: f64 = 1.0;
    let mut radius: usize = 1;
    let step_size: f64 = 1.15;
    let mut lower_bound_check = false;

    // run the multiscale adaptive analysis
    (0..tu).for_each(|s| {
        radius = size_f.floor() as usize;
        single_iteration_2d(
            image_a,
            image_b,
            threshold_a,
            threshold_b,
            result.view_mut(),
            new_tau.view_mut(),
            new_sqrt_n.view_mut(),
            old_tau.view_mut(),
            old_sqrt_n.view_mut(),
            stop.view_mut(),
            radius,
            dn,
            lambda,
            lower_bound_check,
        );
        size_f *= step_size;
        if s == tl {
            lower_bound_check = true;
            let lanes = stop.lanes_mut(Axis(2));
            Zip::from(lanes)
                .and(new_tau.view())
                .and(new_sqrt_n.view())
                .par_for_each(|mut ln, nt, ns| {
                    ln[1] = *nt;
                    ln[2] = *ns;
                });
        }
    });

    Ok(result)
}

/// Single 2-dimensional SACA iteration.
fn single_iteration_2d<T>(
    image_a: ArrayView2<T>,
    image_b: ArrayView2<T>,
    threshold_a: T,
    threshold_b: T,
    mut result: ArrayViewMut2<f64>,
    mut new_tau: ArrayViewMut2<f64>,
    mut new_sqrt_n: ArrayViewMut2<f64>,
    mut old_tau: ArrayViewMut2<f64>,
    mut old_sqrt_n: ArrayViewMut2<f64>,
    mut stop: ArrayViewMut3<f64>,
    radius: usize,
    dn: f64,
    lambda: f64,
    bound_check: bool,
) where
    T: ToFloat64,
{
    // get weighted circle kernel
    let falloff = radius as f64 * (2.5_f64).sqrt();
    let kernel = weighted_circle(radius, falloff, None).unwrap();

    // set up buffers and parameters
    let buf_size = (2 * radius + 1) * (2 * radius + 1);

    // compute weighted kendall's tau and write to output
    let dims_a = image_a.dim();
    let lanes = stop.lanes_mut(Axis(2));
    result
        .indexed_iter_mut()
        .zip(new_tau.iter_mut())
        .zip(new_sqrt_n.iter_mut())
        .zip(lanes)
        .par_bridge()
        .for_each(|(((((row, col), re), nt), nn), mut ln)| {
            // check stop condition and skip loop if true
            if bound_check {
                if ln[0] != 0.0 {
                    return;
                }
            }
            let tau_diff: f64;
            // create buffers for the current local neighborhood
            let mut buf_a = vec![T::default(); buf_size];
            let mut buf_b = vec![T::default(); buf_size];
            let mut buf_w = vec![0.0_f64; buf_size];
            // get the start and end values to fill buffers
            let buf_row_start = get_start_position(row, radius);
            let buf_row_end = get_end_position(row, radius, dims_a.0);
            let buf_col_start = get_start_position(col, radius);
            let buf_col_end = get_end_position(col, radius, dims_a.1);
            fill_buffers_2d(
                image_a,
                image_b,
                kernel.view(),
                old_tau.view(),
                old_sqrt_n.view(),
                &mut buf_a,
                &mut buf_b,
                &mut buf_w,
                dn,
                radius,
                row,
                col,
                buf_row_start,
                buf_row_end,
                buf_col_start,
                buf_col_end,
            );
            // TODO: parallalize this?
            // zero out weights for values below threshold and find the ESS of the neighborhood
            buf_a
                .iter()
                .zip(buf_b.iter())
                .zip(buf_w.iter_mut())
                .for_each(|((&a, &b), w)| {
                    if a < threshold_a || b < threshold_b {
                        *w = 0.0;
                    }
                });
            // find effective sample size
            *nn = effective_sample_size(&buf_w).sqrt();
            if *nn <= 0.0 {
                *nt = 0.0;
                *re = 0.0;
            } else {
                let tau = weighted_kendall_tau_b(&buf_a, &buf_b, &buf_w).unwrap_or(0.0);
                *nt = tau;
                *re = tau * *nn * 1.5;
            }
            if bound_check {
                tau_diff = (ln[1] - *nt).abs() * ln[2];
                if tau_diff > lambda {
                    ln[0] = 1.0;
                    *nt = old_tau[[row, col]];
                    *nn = old_sqrt_n[[row, col]];
                }
            }
        });

    // store old tau and n
    old_tau.assign(&new_tau);
    old_sqrt_n.assign(&new_sqrt_n);
}

/// Fill working buffers from 2-dimensional data.
fn fill_buffers_2d<T>(
    image_a: ArrayView2<T>,
    image_b: ArrayView2<T>,
    kernel: ArrayView2<f64>,
    old_tau: ArrayView2<f64>,
    old_sqrt_n: ArrayView2<f64>,
    buf_a: &mut [T],
    buf_b: &mut [T],
    buf_w: &mut [f64],
    dn: f64,
    radius: usize,
    pos_row: usize,
    pos_col: usize,
    buf_row_start: usize,
    buf_row_end: usize,
    buf_col_start: usize,
    buf_col_end: usize,
) where
    T: ToFloat64,
{
    // set compute parameters
    let mut i: usize = 0;
    let ot = old_tau[[pos_row, pos_col]];
    let on = old_sqrt_n[[pos_row, pos_col]];
    let on_dn = on / dn;
    let pos_row = pos_row as isize;
    let pos_col = pos_col as isize;
    let radius = radius as isize;

    // create iterators for each dimension, zip and iterate
    (buf_row_start..=buf_row_end)
        .flat_map(|r| (buf_col_start..=buf_col_end).map(move |c| (r, c)))
        .for_each(|(r, c)| {
            let tau_diff: f64;
            let tau_diff_abs: f64;
            // subtract current position to get offset from kernel center
            let kr = ((r as isize - pos_row) + radius) as usize;
            let kc = ((c as isize - pos_col) + radius) as usize;
            // load the buffers with data from images and associated weights
            buf_a[i] = image_a[[r, c]];
            buf_b[i] = image_b[[r, c]];
            buf_w[i] = kernel[[kr, kc]];
            tau_diff = old_tau[[r, c]] - ot;
            tau_diff_abs = tau_diff.abs() * on_dn;
            if tau_diff_abs < 1.0 {
                buf_w[i] = buf_w[i] * (1.0 - tau_diff_abs).powi(2);
            } else {
                buf_w[i] = 0.0;
            }
            i += 1;
        });

    // zero out the rest of the buffers
    buf_a[i..].fill(T::default());
    buf_b[i..].fill(T::default());
    buf_w[i..].fill(0.0);
}

/// Get the end position for filling the buffers along an axis.
fn get_end_position(location: usize, radius: usize, boundary: usize) -> usize {
    let end = location + radius;
    if end >= boundary { boundary - 1 } else { end }
}

/// Get the start position for filling the buffers along an axis.
fn get_start_position(location: usize, radius: usize) -> usize {
    if location < radius {
        0
    } else {
        location - radius
    }
}
