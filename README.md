# imgal

⚠️ Warning: This is an experimental project!

Imgal (**Im**a**g**e **A**lgorithm **L**ibrary), is fast and mostly* safe image processing and algorithm library written in Rust. This library is directly inspired by [imagej-ops](https://github.com/imagej/imagej-ops/),
[SciJava Ops](https://github.com/scijava/scijava), [ImgLib2](https://github.com/imglib/imglib2), the ImageJ2 ecosystem. `imgal` is organized as a monorepo with `imgal` as the core library that contains the algorithm logic while `imgal_java` and `imgal_python` serve `imgal`'s
Java and Python language bindings.

## Installation

### Building `imgal` from source

You can build the entire project from the root with:

```bash
$ cargo build --release
```
> [!NOTE]
>
> `--release` is _necessary_ to compile speed optimized libraries. Without this flag `rustc` will retain debug symbols (among other things) that reduce the performance of the libraries.

This will create one Rust static library (`.rlib`) file for `imgal` and two shared library files for the Java and Python bindings respectively. The file extension of the shared library is operating system dependent:

| Platform | Extension |
| :---     | :---      |
| Linux    | `.so`     |
| macOS    | `.dylib`  |
| Windows  | `.dll`    |

Additionally, shared libraries will be prefixed with `lib`, making the compiled `imgal` library filename `libimgal.rlib`. After building `imgal` the three library files can be found in `target/release`.

| File name | Description |
| :---      | :---        |
| libimgal | The main Rust static library.
| libimgal_java | Java bindings using the Foreign Function and Memory (FFM) API (targeting Java 22+). |
| libimgal_python | Python bindings (using PyO3). |


### Building `imgal_python` from source

To build the `imgal` Python bindings, use the `maturin` build tool. If you're using `uv` you can do the following in the `imgal_python`:

```bash
$ cd imgal_python
$ uv run maturin develop --release
```

This will create a `.venv` in the local directory, compile `imgal` and `imgal_python` and install the bindings in the venv.

Alternatively if you're using `conda` or `mamba` you can do the following:

```bash
$ cd imgal_python
$ mamba activate myenv
(myenv) $ mamba install maturin
...
(myenv) $ maturin develop --release
```


## Usage

### Using `imgal` in Python

```python
import imgal.simulation.decay as dsim
import imgal.simulation.noise as nsim

# biexponential decay parameters
samples = 256
period = 12.5
taus = [1.0, 3.0]
fractions = [0.7, 0.3]
photons = 5000.0
irf_center = 3.0
irf_width = 0.5
shape = (75, 50)

# simulate 3D biexponential decay data, (74, 50, 256)
sim = dsim.gaussian_exponential_3d(samples, period, taus, fractions, photons, irf_center, irf_width, shape)

# simulate poisson noise in place
nsim.poisson_3d_mut(sim, 0.8)
```
