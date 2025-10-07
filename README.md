# imgal

![crates.io](https://img.shields.io/crates/v/imgal.svg)

Imgal (**Im**a**g**e **A**lgorithm **L**ibrary) is a fast and open-source scientific image processing and algorithm library. This library is directly inspired by [imagej-ops](https://github.com/imagej/imagej-ops/),
[SciJava Ops](https://github.com/scijava/scijava), [ImgLib2](https://github.com/imglib/imglib2), the ImageJ2 ecosystem. The `imgal` library aims to offer users access to fast and well documented image algorithms.
`imgal` is organized as a monorepo with `imgal` as the core library that contains the algorithm logic while `imgal_java` and `imgal_python` serve `imgal`'s
Java and Python language bindings respectively.

## Installation

### Building `imgal` from source

You can build the entire project from the root with:

```bash
$ cargo build --release
```
> [!NOTE]
>
> `--release` is _necessary_ to compile speed optimized libraries and utilize compiler optimizations.

This will create one Rust static library (`.rlib`) file for `imgal` and two shared library files for the Java and Python bindings respectively. The file extension of the shared library is operating system dependent:

| Platform | Extension |
| :---     | :---      |
| Linux    | `.so`     |
| macOS    | `.dylib`  |
| Windows  | `.dll`    |

Additionally, shared libraries will be prefixed with `lib`, making the compiled `imgal` library filename `libimgal.rlib`. After building `imgal` the three library files can be found in `target/release`.

| File name | Description |
| :---      | :---        |
| libimgal.rlib | The main Rust static library.
| libimgal.so | Python bindings (using PyO3). |
| libimgal_java.so | Java bindings using the Foreign Function and Memory (FFM) API (targeting Java 22+). |


### Building `imgal_python` from source

To build the `imgal` Python bindings, use the `maturin` build tool. If you're using `uv` you can do the following in the `imgal_python` crate directory:

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

### Using `imgal` with Rust

Add `imgal` do your crates's dependencies and import the `imgal` namespace with your desired function. The example below demonstrates how to create a cube shaped kernel with a weighted sphere (_i.e._ the neighborhood) of the specified radius and weight decay rate defined by the falloff radius.

```rust
use imgal::kernel::neighborhood;

fn main() {
  // set radius and weight decay falloff radius
  let radius = 5;
  let falloff = 7.5;

  // create a weighted sphere with given radius and falloff
  let sphere = neighborhood::weighted_sphere(radius, falloff, None);
}
```

### Using `imgal` with Python

Once `imgal_python` has been installed in a compatiable Python environment, `imgal` will be available to import. The example below demonstrates how to obtain a colocalization z-score (_i.e._ colocalization and
anti-colocalization strength) using the [Spatially Adaptive Colocalization Analysis (SACA)](https://doi.org/10.1109/TIP.2019.2909194) framework. The two number values after the channels are threshold values for channels `a` and `b` respectively.

```python
import imgal.colocalization as coloc
from tifffile import imread

# load some data
image = imread("path/to/data.tif")

# slice channels to perform colocalization analysis
ch_a = image[:, :, 0]
ch_b = image[:, :, 1]

# perform SACA 2D
coloc_zscore = coloc.saca_2d(ch_a, ch_b, 500.0, 500.0)
```
