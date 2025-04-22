package org.imgal.phasor;

/**
 * Interface for native time domain phasor.
 *
 * @author Edward Evans
 */
class TimeDomain {

	// load the imgal native library
	static {
		System.loadLibrary("imgal");
	}

	// declare native "time domain" functions
	private static native double nativeImaginary(double[] y, double period, double harmonic, double omega);
	private static native double nativeReal(double[] y, double period, double harmonic, double omega);

	/**
	 * TODO
	 */
	public static double imaginary(final double[] y,
			final double period,
			final double harmonic,
			final double omega) {
		return TimeDomain.nativeImaginary(y, period, harmonic, omega);
	}

	/**
	 * TODO
	 */
	public static double real(final double[] y,
			final double period,
			final double harmonic,
			final double omega) {
		return TimeDomain.nativeReal(y, period, harmonic, omega);
	}
}
