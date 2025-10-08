# pyimgal

[![pypi](https://img.shields.io/pypi/v/pyimgal)](https://pypi.org/project/pyimgal/)

The `pyimgal` package provides Rust bindings for Python for [imgal](https://github.com/imgal-sc/imgal).

## Installation

### Get `pyimgal` from PyPI

You can install `pyimgal` from PyPI with:

```bash
pip install pyimgal
```

The `pyimgal` package supports the following architectures for Python `3.9`,
`3.10`, `3.11`, `3.12`, and `3.13`:

| Operating System | Architecture |
| :---             | :---                 |
| Linux            | amd64, aarch64       |
| macOS            | intel, arm64         |
| Windows          | amd64                |

Alternatively you can install `pyimagal` from source by building the `imgal_python`
repository.

### Build `pyimgal` from source

To build the `pyimgal` Python package from source, use the `maturin` build tool
(this requires the Rust toolchain). If you're using `uv` to manage your Python
virtual environments (venv) add `maturin` to your environment and run the
`maturin develop --release` command in the `imgal_python` directory of the
[imgal](https://github.com/imgal-sc/imgal) repository with your venv activated:

```bash
$ source ~/path/to/myenv/.venv/bin/activate
$ (myenv) cd imgal_python
$ maturin develop --release
```

Alernatively if you're using `conda` or `mamba` you can do the following:

```bash
$ cd imgal_python
$ mamba activate myenv
(myenv) $ mamba install maturin
...
(myenv) $ maturin develop --release
```

This will install `pyimgal` in the currently active Python environment.

### Using `pyimgal`

Once `pyimgal` has been installed in a compatiable Python environment, `imgal`
will be available to import. The example below demonstrates how to obtain a
colocalization z-score (_i.e._ colocalization and anti-colocalization strength)
using the [Spatially Adaptive Colocalization Analysis (SACA)](https://doi.org/10.1109/TIP.2019.2909194)\
framework. The two number values after the channels are threshold values for
channels `a` and `b` respectively.

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
