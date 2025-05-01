use std::slice;

use imgal_core::statistics;

// C ABI inteface for imgal_core::statistics::sum.
#[unsafe(no_mangle)]
pub extern "C" fn sum(ptr: *const f64, len: usize) -> f64 {
    // saftey check: validate the pointer and array length
    if ptr.is_null() || len == 0 {
        return 0.0;
    }
    // create a slice and compute sum
    let s = unsafe { slice::from_raw_parts(ptr, len) };
    statistics::sum(&s)
}
