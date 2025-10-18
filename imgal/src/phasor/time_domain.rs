use std::f64;

use ndarray::{Array2, Array3, ArrayView2, ArrayView3, Axis, Zip, stack};

use crate::error::ArrayError;
use crate::integration::midpoint;
use crate::parameter::omega;
use crate::traits::numeric::ToFloat64;

/// Compute the histogram quality value from a 1-dimensional decay array.
///
/// # Description
///
/// This function computes a weighted quality metric, `q`, for time domain
/// fluorescence lifetime imaging microscopy (FLIM) photon arrival histograms.
/// Bins exceeding the `count_threshold` contribute a weighted value to `q`,
/// rewarding bins with high counts, while still considering histograms with
/// their counts distributed across multiple valid bins. The `q` value provides
/// a measure of photon counts above threshold and the count distribution
/// quality. The histogram quality metric is defined as:
///
/// ```text
/// q = (vb / n²) * ∑₍xᵢ > t₎ xᵢ²
/// ```
///
/// where:
/// - `n` is the total number of bins.
/// - `xᵢ` is the value in bin `i`.
/// - `vb` is the number of valid bins (_i.e._ bins above threshold).
/// - `t` is the count threshold.
/// - `I(·)` is the indicator function, returns `true` if `xᵢ` is above the
///    threshold, else it returns `false`.
///
/// # Arguments
///
/// * `data`: The 1-dimensional decay data as a slice.
/// * `count_threshold`: The minimum bin count value a bin must exceed to be
///    considered valid.
///
/// # Returns
///
/// * `f64`: The histogram quality value, `q`. A `q` value of 0.0 occurs when
///    no bins exceed the threshold, indicating an insufficient histogram for
///    analysis. Values in the range of < 1.0 indicate histograms with few
///    photons overall and/or photons disributed across many other bins with
///    low counts per bin. Values between 1.0 and 10.0 contain good photon counts
///    and/or photon distribution across the histogram. Values exceeding 10.0
///    indicate high quality histograms with high photon counts that are
///    distributed across the histogram.
pub fn histogram_quality<T>(data: &[T], count_threshold: T) -> f64
where
    T: ToFloat64,
{
    let vb = data.iter().fold(0_i32, |mut acc, &v| {
        if v > count_threshold {
            acc += 1;
        }
        acc
    }) as f64;
    let dl = data.len() as f64;
    let q = data.iter().fold(0.0_f64, |mut acc, &v| {
        if v > count_threshold {
            acc += v.to_f64().powi(2);
        }
        acc
    });

    q * (vb / dl.powi(2))
}

/// Compute a histogram quality image from a 3-dimensional decay array.
///
/// # Description
///
/// This function computes a weighted quality metric, `q`, for time domain
/// fluorescence lifetime imaging microscopy (FLIM) photon arrival 3-dimensional
/// arrays. Bins exceeding the `count_threshold` contribute a weighted value to
/// `q`, rewarding bins with high counts, while still considering histograms
/// with their counts distributed across multiple valid bins. The `q` value
/// provides a measure of photon counts above threshold and the count
/// distribution quality. This function returns the `q` values as a
/// 2-dimensional array or `q` map. The histogram quality metric is defined as:
///
/// ```text
/// q = (vb / n²) * ∑₍xᵢ > t₎ xᵢ²
/// ```
///
/// where:
/// - `n` is the total number of bins.
/// - `xᵢ` is the value in bin `i`.
/// - `vb` is the number of valid bins (_i.e._ bins above threshold).
/// - `t` is the count threshold.
/// - `I(·)` is the indicator function, returns `true` if `xᵢ` is above the
///    threshold, else it returns `false`.
///
/// # Arguments
///
/// * `data`: The 3-dimensional decay data.
/// * `count_threshold`: The minimum bin count value a bin must exceed to be
///    considered valid.
/// * `axis`: The decay or lifetime axis, default = 2.
///
/// # Returns
///
/// * `Ok(Array2<f64>)`: The 2-dimensional pixel-wise histogram quality, `q`,
///    value map. A `q` value of 0.0 occurs when no bins exceed the threshold,
///    indicating an insufficient histogram for analysis. Values in the range
///    of < 1.0 indicate histograms with few photons overall and/or photons
///    disributed across many other bins with low counts per bin. Values
///    between 1.0 and 10.0 contain good photon counts and/or photon
///    distribution across the histogram. Values exceeding 10.0 indicate high
///    quality histograms with high photon counts that are distributed across
///    the histogram.
/// * `Err(ArrayError)`: If axis is >= 3.
pub fn histogram_quality_image<T>(
    data: ArrayView3<T>,
    count_threshold: T,
    axis: Option<usize>,
) -> Result<Array2<f64>, ArrayError>
where
    T: ToFloat64,
{
    // set optional parameter if needed
    let a = axis.unwrap_or(2);

    // check if axis parameter is valid
    if a >= 3 {
        return Err(ArrayError::InvalidAxis {
            axis_idx: a,
            dim_len: 3,
        });
    }

    // create output array and zip iterate
    let mut shape = data.shape().to_vec();
    shape.remove(a);
    let mut q_arr = Array2::<f64>::zeros((shape[0], shape[1]));
    let lanes = data.lanes(Axis(a));
    Zip::from(lanes)
        .and(q_arr.view_mut())
        .par_for_each(|ln, p| {
            if let Some(l) = ln.as_slice() {
                *p = histogram_quality(l, count_threshold);
            } else {
                *p = histogram_quality(&ln.to_vec(), count_threshold);
            }
        });

    Ok(q_arr)
}

/// Compute the real and imaginary (G, S) coordinates of a 3-dimensional decay
/// image.
///
/// # Description
///
/// The real (G) and imaginary (S) components are calculated using the normalized
/// sine and cosine Fourier transforms:
///
/// ```text
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
/// ```
///
/// # Arguments
///
/// * `data`: I(t), the decay data image.
/// * `period`: The period (_i.e._ time interval).
/// * `harmonic`: The harmonic value, default = 1.0.
/// * `axis`: The decay or lifetime axis, default = 2.
///
/// # Returns
///
/// * `Ok(Array3<f64>)`: The real and imaginary coordinates as a 3D (ch, row, col) image,
///    where G and S are indexed at 0 and 1 respectively on the _channel_ axis.
/// * `Err(ArrayError)`: If axis is >= 3.
pub fn image<T>(
    data: ArrayView3<T>,
    period: f64,
    mask: Option<ArrayView2<bool>>,
    harmonic: Option<f64>,
    axis: Option<usize>,
) -> Result<Array3<f64>, ArrayError>
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let h = harmonic.unwrap_or(1.0);
    let a = axis.unwrap_or(2);

    // check if axis parameter is valid
    if a >= 3 {
        return Err(ArrayError::InvalidAxis {
            axis_idx: a,
            dim_len: 3,
        });
    }

    // initialize phasor parameters
    let w = omega(period);
    let n: usize = data.len_of(Axis(a));
    let dt: f64 = period / n as f64;
    let h_w_dt: f64 = h * w * dt;

    // initialize buffers
    let mut w_cos_buf: Vec<f64> = Vec::with_capacity(n);
    let mut w_sin_buf: Vec<f64> = Vec::with_capacity(n);

    // drop specified axis and create new G and S output arrays with new shape
    let mut shape = data.shape().to_vec();
    shape.remove(a);
    let mut g_arr = Array2::<f64>::zeros((shape[0], shape[1]));
    let mut s_arr = Array2::<f64>::zeros((shape[0], shape[1]));

    // load the waveform buffers
    for i in 0..n {
        w_cos_buf.push(f64::cos(h_w_dt * (i as f64)));
        w_sin_buf.push(f64::sin(h_w_dt * (i as f64)));
    }

    // compute phasor coordinates per lane, optionally only in mask area
    let lanes = data.lanes(Axis(a));
    if let Some(msk) = mask {
        Zip::from(lanes)
            .and(msk)
            .and(&mut g_arr)
            .and(&mut s_arr)
            .par_for_each(|ln, m, g, s| {
                if *m {
                    let mut iv = 0.0;
                    let mut gv = 0.0;
                    let mut sv = 0.0;
                    ln.iter()
                        .zip(w_cos_buf.iter())
                        .zip(w_sin_buf.iter())
                        .for_each(|((v, cosv), sinv)| {
                            // midpoint integration
                            let vf: f64 = (*v).to_f64();
                            iv += vf;
                            gv += vf * cosv;
                            sv += vf * sinv;
                        });
                    // midpoint integration, multiply by data point width
                    iv *= dt;
                    gv *= dt;
                    sv *= dt;
                    // normalize G/S values and write to output arrays
                    *g = gv / iv;
                    *s = sv / iv;
                } else {
                    // if false on mask, set G/S output to zero
                    *g = 0.0;
                    *s = 0.0;
                }
            });
    } else {
        // compute phasor coordinates per lane in the entire array, no mask
        Zip::from(&mut g_arr)
            .and(&mut s_arr)
            .and(lanes)
            .par_for_each(|g, s, ln| {
                let mut iv = 0.0;
                let mut gv = 0.0;
                let mut sv = 0.0;
                ln.iter()
                    .zip(w_cos_buf.iter())
                    .zip(w_sin_buf.iter())
                    .for_each(|((v, cosv), sinv)| {
                        // midpoint integration
                        let vf: f64 = (*v).to_f64();
                        iv += vf;
                        gv += vf * cosv;
                        sv += vf * sinv;
                    });
                // midpoint integration, multiply by data point width
                iv *= dt;
                gv *= dt;
                sv *= dt;
                // normalize G/S values and write to output arrays
                *g = gv / iv;
                *s = sv / iv;
            });
    }

    // stack G and S arrays, (row, col, ch)
    Ok(stack(Axis(2), &[g_arr.view(), s_arr.view()]).unwrap())
}

/// Compute the imaginary (S) component of a 1-dimensional decay curve.
///
/// # Description
///
/// The imaginary (S) component is calculated using the normalized sine Fourier
/// transform:
///
/// ```text
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
/// ```
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `data`: I(t), the 1-dimensonal decay curve.
/// * `period`: The period (_i.e._ time interval).
/// * `harmonic`: The harmonic value, default = 1.0.
///
/// # Returns
///
/// * `f64`: The imaginary component, S.
pub fn imaginary<T>(data: &[T], period: f64, harmonic: Option<f64>) -> f64
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega(period);

    // integrate sine transform (imaginary)
    let n: usize = data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(data[i].to_f64() * f64::sin(h_w_dt * (i as f64)));
    }
    let i_sin_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(data, Some(dt));
    i_sin_integral / i_integral
}

/// Compute the real (G) component of a 1-dimensional decay curve.
///
/// # Description
///
/// The real (G) component is calculated using the normalized cosine Fourier
/// transform:
///
/// ```text
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
/// ```
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `data`: I(t), the 1-dimensional decay curve.
/// * `period`: The period, (_i.e._ time interval).
/// * `harmonic`: The harmonic value, default = 1.0.
///
/// # Returns
///
/// * `f64`: The real component, G.
pub fn real<T>(data: &[T], period: f64, harmonic: Option<f64>) -> f64
where
    T: ToFloat64,
{
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega(period);

    // integrate cosine transform (real)
    let n: usize = data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(data[i].to_f64() * f64::cos(h_w_dt * (i as f64)));
    }
    let i_cos_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(data, Some(dt));
    i_cos_integral / i_integral
}
