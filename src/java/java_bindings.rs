use jni::JNIEnv;
use jni::objects::{JClass, JDoubleArray, ReleaseMode};
use jni::sys::{jdouble};

use crate::statistic::sum;

// Java binding for double array, no copy
#[no_mangle]
pub extern "system" fn Java_org_imgal_statistic_NativeSum_nativeSum(
    mut env: JNIEnv,
    _class: JClass,
    array: JDoubleArray
    ) -> jdouble {
    // get unsafe array elements
    let elements = unsafe {
        env.get_array_elements(&array, ReleaseMode::NoCopyBack)
    }.unwrap();

    // compute array sum
    sum(&elements)
}
