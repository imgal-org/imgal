# imgal

⚠️ Warning: This is an experimental project!

Imgal (**Im**a**g**e **A**lgorithm **L**ibrary), is fast and mostly* safe image processing and algorithm library written in Rust. This library is directly inspired by imagej-ops,
SciJava Ops, imglib2, the ImageJ2 ecosystems. Imgal is organized as a monorepo with `imgal-core` as the base alorithm library while `imgal-java` and `imgal-python` serve `imgal`'s
Java and Python bindings respectively.

# Building `imgal` from source

You can build the entire project from the root with:

```bash
$ cargo build --release
```

This will create one `.rlib` file for `imgal-core` and two `.so` on Linux systems (or on macOS `.dylib` and `.dll` on Windows) files in `target/release`. For example on a Linux system:

- `libimgal_core.rlib` -> The main Rust static library.
- `libimgal_java.so` -> Java bindings using the Foreign Function and Memory (FFM) API (targeting Java 22+).
    - The Java side of these bindings is built with `maven`.
- `libimgal_python.so` -> Python bindings using PyO3 (note that the Python bindings are built in the `imgal-python` subdirectory with `maturin`.

See the specific bindings `README.md` file for specific instructions on how to build, install and use the bindings.
