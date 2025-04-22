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
	private static native double nativeAbbeDiffractionLimit(double wavelength, double na);

	/**
	 * TODO
	 */
	public static double abbeDiffractionLimit(final double wavelength, final double na) {
		return Parameters.nativeAbbeDiffractionLimit(wavelength, na);
	}

	/**
	 * TODO
	 */
	public static double omega(final double period) {
		return Parameters.nativeOmega(period);
	}
}
