use std::f64;

use ndarray::{Array2, Array3, ArrayView3, Axis, Zip, stack};

use crate::integrate::midpoint;
use crate::parameters;
use crate::traits::numeric::FloatLike;

/// Compute the real and imaginary (G, S) coordinates of a
/// 3D decay image.
///
/// # Description
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// # Arguments
///
/// * `i_data`: I(t), the decay data image (time, row, col).
/// * `period`: The period in seconds.
/// * `harmonic`: The harmonic value, default = 1.0.
/// * `omega`: The angular frequency, default = computed from the period.
///
/// # Returns
///
/// * `Array3<f64>`: The real and imaginary coordinates as a 3D (ch, row, col) image,
///     where G and S are indexed at 0 and 1 respectively on the _channel_ axis.
pub fn image<T>(
    i_data: &ArrayView3<T>,
    period: T,
    harmonic: Option<T>,
    omega: Option<T>,
) -> Array3<f64>
where
    T: FloatLike,
{
    // set optional parameters if needed
    let h = harmonic.unwrap_or(T::from_f64(1.0));
    let w = omega.unwrap_or_else(|| T::from_f64(parameters::omega(period)));

    // initialize phasor parameters
    let n: usize = i_data.len_of(Axis(2));
    let dt: f64 = period.into() / n as f64;
    let h_w_dt: f64 = h.into() * w.into() * dt;

    // initialize buffers
    let mut w_cos_buf: Vec<f64> = Vec::with_capacity(n);
    let mut w_sin_buf: Vec<f64> = Vec::with_capacity(n);

    // initialize output array
    let shape = i_data.dim();
    let mut g_arr = Array2::<f64>::zeros((shape.0, shape.1));
    let mut s_arr = Array2::<f64>::zeros((shape.0, shape.1));

    // load the waveform buffers
    for i in 0..n {
        w_cos_buf.push(f64::cos(h_w_dt * (i as f64)));
        w_sin_buf.push(f64::sin(h_w_dt * (i as f64)));
    }

    // compute phasor coordinates per lane
    let lanes = i_data.lanes(Axis(2));
    Zip::from(&mut g_arr)
        .and(&mut s_arr)
        .and(lanes)
        .par_for_each(|g, s, ln| {
            let mut iv: f64 = 0.0;
            let mut gv: f64 = 0.0;
            let mut sv: f64 = 0.0;
            let l = ln.as_slice().unwrap();
            l.iter()
                .zip(w_cos_buf.iter())
                .zip(w_sin_buf.iter())
                .for_each(|((v, cosv), sinv)| {
                    // deref value, "v", and convert to f64 for compute
                    let vf: f64 = (*v).into();
                    // midpoint integration, sum the midpoints
                    iv += vf;
                    gv += vf * cosv;
                    sv += vf * sinv;
                });
            // midpoint integration, multiply by width between data points
            iv *= dt;
            gv *= dt;
            sv *= dt;
            // normalize and write G/S values to arrays
            *g = gv / iv;
            *s = sv / iv;
        });

    // stack G and S arrays, (row, col, ch)
    stack(Axis(2), &[g_arr.view(), s_arr.view()]).unwrap()
}

/// Compute the imaginary S component of lifetime data.
///
/// # Description
///
/// The imaginary component, S, time domain equation is calculated
/// using:
///
/// S = ∫(I(t) * sin(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `i_data`: I(t), the decay data slice.
/// * `period`: The period in seconds.
/// * `harmonic`: The harmonic value, default = 1.0.
/// * `omega`: The angular frequency, default = computed from the period.
///
/// # Returns
///
/// * `f64`: The imaginary, S, component.
pub fn imaginary(i_data: &[f64], period: f64, harmonic: Option<f64>, omega: Option<f64>) -> f64 {
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(|| parameters::omega(period));

    // integrate sine transform (imaginary)
    let n: usize = i_data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(i_data[i] * f64::sin(h_w_dt * (i as f64)));
    }
    let i_sin_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(&i_data, Some(dt));
    i_sin_integral / i_integral
}

/// Compute the real G component of lifetime data.
///
/// # Description
///
/// The real component, G, time domain equation is calculated
/// using:
///
/// G = ∫(I(t) * cos(nωt) * dt) / ∫(I(t) * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// * `i_data`: I(t), the decay data slice.
/// * `period`: The period in seconds.
/// * `harmonic`: The harmonic value, default = 1.0.
/// * `omega`: The angular frequency, default = computed from the period.
///
/// # Returns
///
/// * `f64`: The real, G, component.
pub fn real(i_data: &[f64], period: f64, harmonic: Option<f64>, omega: Option<f64>) -> f64 {
    // set optional parameters if needed
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(|| parameters::omega(period));

    // integrate cosine transform (real)
    let n: usize = i_data.len();
    let dt: f64 = period / (n as f64);
    let h_w_dt: f64 = h * w * dt;
    let mut buf = Vec::with_capacity(n);
    for i in 0..n {
        buf.push(i_data[i] * f64::cos(h_w_dt * (i as f64)));
    }
    let i_cos_integral: f64 = midpoint(&buf, Some(dt));
    let i_integral: f64 = midpoint(&i_data, Some(dt));
    i_cos_integral / i_integral
}
