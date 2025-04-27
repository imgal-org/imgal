package org.imgal;

import java.lang.foreign.Arena;
import java.lang.foreign.Linker;
import java.lang.foreign.SymbolLookup;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

/**
 * Abstract class for all Java bindings.
 *
 * @author Edward Evans
 */
public abstract class AbstractNativeLibrary {
	// TODO support macOS and Windows style native libs
	private static String libPath = "/native/libimgal.so";
	public static final SymbolLookup libLookup;
	public static final Linker linker = Linker.nativeLinker();

	// copy the Rust library from resources and then load it (for SymbolLookup)
	static {
		try {
			URL url = AbstractNativeLibrary.class.getResource(libPath);
			Path tmpLib = Files.createTempFile("libimgal", "so");
			try (InputStream is = url.openStream()) {
				Files.copy(is, tmpLib, StandardCopyOption.REPLACE_EXISTING);
			}
			tmpLib.toFile().deleteOnExit();
			libLookup = SymbolLookup.libraryLookup(tmpLib, Arena.global());
		} catch (Exception e) {
			throw new RuntimeException("Failed to load library", e);
		}
	}
}
