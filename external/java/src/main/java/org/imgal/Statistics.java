package org.imgal;

/**
 * Interface for native statistics
 *
 * @author Edward Evans
 */
class Statistics {

	// load the imgal Rust library
	static {
		System.loadLibrary("imgal");
	}

	// declare native library functions
	private static native double nativeSum(double[] input);

	/**
	 * TODO
	 */
	public static double sum(final double[] input) {
		return NativeSum.nativeSum(input);
	}
}
