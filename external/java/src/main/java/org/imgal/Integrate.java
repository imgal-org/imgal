package org.imgal;

/**
 * Interface for native integration.
 *
 * @author Edward Evans
 */
class Integrate {

	// load the imgal native library
	static {
		System.loadLibrary("imgal");
	}

	// declare native "integrate" functions
	private static native double nativeCompositeSimpson(double[] y, double deltaX);
	private static native double nativeMidpoint(double[] y, double h);
	private static native double nativeSimpson(double[] y, double deltaX);

	/**
	 * TODO
	 */
	public static double compositeSimpson(final double[] y, final double deltaX) {
		return Integrate.nativeCompositeSimpson(y, deltaX);
	}

	/**
	 * TODO
	 */
	public static double midpoint(final double[] y, final double h) {
		return Integrate.nativeMidpoint(y, h);
	}

	/**
	 * TODO
	 */
	public static double simpson(final double[] y, final double deltaX) {
		return Integrate.nativeSimpson(y, deltaX);
	}
}
