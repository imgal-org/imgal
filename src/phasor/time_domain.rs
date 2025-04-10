use crate::parameters;

// TODO imaginary compute

/// Compute the real or G component of lifetime data.
///
/// The real, G, component time domain equation is calculated
/// using the following transformation using the trapezodial rule:
///
/// G = ∫(data * cos(nωt) * dt) / ∫(data * dt)
///
/// Where 'n' and 'ω' are harmonic and omega values respectively.
///
/// # Arguments
///
/// # Returns
pub fn real(
    input: &[f64],
    period: f64,
    harmonic: Option<f64>,
    omega: Option<f64>,
    integration_time: Option<&[f64]>,
) -> f64{
    // compute and set optional paramters if needed
    // Should these two be refs?
    let h: f64 = harmonic.unwrap_or(1.0);
    let w: f64 = omega.unwrap_or_else(|| parameters::omega(&period));
    ///////////////
    let use_buffer: bool;
    let extern_int_time: &[f64];
    let local_int_time: Vec<f64>;

    // set if using a buffer (i.e. a ref to int time)
    match integration_time {
        Some(time_array) => {
            extern_int_time = time_array;
            use_buffer = true;
        },
        None => {
            local_int_time = calculate_integration_time(input.len(), &period);
            use_buffer = false;
        }
    }
    ///////////////
    // unwrap to ref slice. check if none.
    // if none calculate the buffer ourselves to a new var
    // use a match!
    // compute the "real" integral
    let mut sum_data: f64 = 0.0;
    let mut g: f64 = 0.0;
    for (i, &value) in input.enumerate() {
        sum_data += value;
        g += value * f64::cos(h * w * t[i]);
    }
    g /= sum_data
}

fn calculate_integration_time(
    bins: usize,
    period: &f64,
) -> Vec<f64> {
    let mut time = vec![0.0; bins];
    let dt: f64 = period / bins as f64;
    for i in 0..bins {
        time[i] = i as f64 * dt;
    }
    time
}
