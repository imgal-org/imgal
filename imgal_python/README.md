# pyimgal

The `pyimgal` package serves as the Python bindings wrapper for [imgal](https://github.com/imgal-sc/imgal).

## Installation

### `pyimgal` from PyPI

You can install `pyimgal` from PyPI with:

```bash
pip install pyimgal
```

### `imgal_python` from source

To build the `pyimgal` Python bindings from source, use the `maturin` build tool. If you're using `uv` you can do the following in the `imgal_python` crate directory to build the Python bindings:

```bash
$ cd imgal_python
$ uv run maturin develop --release
```

This will create a `.venv` in the local directory, compile the `imgal` Rust library and the `imgal_python` PyO3 bindings and install the bindings in the venv as `pyimgal`.

Alternatively if you're using `conda` or `mamba` you can do the following:

```bash
$ cd imgal_python
$ mamba activate myenv
(myenv) $ mamba install maturin
...
(myenv) $ maturin develop --release
```

### Using `imgal` with Python

Once `pyimgal` has been installed in a compatiable Python environment, `imgal` will be available to import. The example below demonstrates how to obtain a colocalization z-score (_i.e._ colocalization and
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
