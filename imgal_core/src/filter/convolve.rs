use ndarray::{Array1, ArrayView1};
use rustfft::{FftPlanner, num_complex::Complex, num_traits::Zero};

/// Convolve two 1-dimensional signals using the Fast Fourier Transform (FFT).
///
/// # Description
///
/// Compute the convolution of two discrete signals (`a` and `b`) by transforming
/// them to the frequency domain, multiplying them, and then transforming the
/// result back into a signal.
///
/// # Arguments
///
/// * `a`: The first input signal to FFT convolve. Typically the "data" signal
///   or the longest of the two signals.
/// * `b`: The second input signal to FFT convolve. Typically a kernel or instrument
///   response function to convolve with.
///
/// # Returns
///
/// * `Array1<f64>`: The FFT convolved result of the same length as input signal
///   `a`.
pub fn fft_convolve(a: ArrayView1<f64>, b: ArrayView1<f64>) -> Array1<f64> {
    // compute FFT size
    let n_a = a.len();
    let n_b = b.len();
    let n_fft = n_a + n_b - 1;
    let fft_size = n_fft.next_power_of_two();

    // allocate buffers
    let mut a_fft_buf = vec![Complex::zero(); fft_size];
    let mut b_fft_buf = vec![Complex::zero(); fft_size];

    // fill arrays with input data
    a_fft_buf[..n_a].iter_mut().enumerate().for_each(|(i, v)| {
        *v = Complex::new(a[i], 0.0);
    });
    b_fft_buf[..n_b].iter_mut().enumerate().for_each(|(i, v)| {
        *v = Complex::new(b[i], 0.0);
    });

    // create FFT planner
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_size);
    let ifft = planner.plan_fft_inverse(fft_size);

    // compute foward FFTs
    fft.process(&mut a_fft_buf);
    fft.process(&mut b_fft_buf);

    // multiply in the frequency domain
    a_fft_buf.iter_mut().enumerate().for_each(|(i, v)| {
        *v = *v * b_fft_buf[i];
    });

    // compute inverse FFT
    ifft.process(&mut a_fft_buf);

    // extract real component, scale and trim to input length
    let scale = 1.0 / fft_size as f64;
    let mut result = vec![0.0; n_a];
    result.iter_mut().enumerate().for_each(|(i, v)| {
        *v = a_fft_buf[i].re * scale;
    });
    Array1::from_vec(result)
}
