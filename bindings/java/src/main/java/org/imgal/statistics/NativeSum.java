package org.imgal.statistics;

/**
 * Interface for native statistics
 *
 * @author Edward Evans
 */
class NativeSum {

	// load the imgal Rust library
	static {
		System.loadLibrary("imgal");
	}

	// declare native library functions
	private static native double nativeSum(double[] array);

	public static double sum(final double[] array) {
		return NativeSum.nativeSum(array);
	}

	public static void main(String[] args) {
		double[] input = {1.0, 5.0, 10.0};
		System.out.println(NativeSum.sum(input));
	}
}
