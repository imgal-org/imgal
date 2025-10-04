# imgal_python

The `imgal_python` package serves as the Python bindings wrapper for [imgal](https://github.com/imgal-sc/imgal).

## Installation

### `imgal` from PyPI

You can install `imgal` from PyPI with:

```bash
pip install imgal
```

### `imgal` from source

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
