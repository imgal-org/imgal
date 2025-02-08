use jni::JNIEnv;
use jni::objects::{JClass, JDoubleArray};
use jni::sys::{jdouble};

use crate::math::sum;

// binding for double array
#[no_mangle]
pub extern "system" fn Java_TestRustJavaInterop_sum(
    env: JNIEnv,
    _class: JClass,
    input: JDoubleArray
    ) -> jdouble {
    // create a rust array
    let len = env.get_array_length(&input).unwrap();
    // copy java data into rust array
    let mut buf: Vec<f64> = vec![0.0; len as usize];
    let _ = env.get_double_array_region(input, 0, &mut buf);
    // compute sum
    let output = sum(&buf);
    // return sum result
    output
}
