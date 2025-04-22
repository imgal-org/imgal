package org.imgal;

/**
 * Interface for native parameters.
 *
 * @author Edward Evans
 */
class Parameters {

	// load the imgal native library
	static {
		System.loadLibrary("imgal");
	}

	// declate native "parameters" functions
	private static native double nativeOmega(double period);

	/**
	 * TODO
	 */
	public static double omega(final double period) {
		return Parameters.nativeOmega(period);
	}
}
