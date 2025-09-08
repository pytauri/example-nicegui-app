# Build Standalone App

> ref: <https://pytauri.github.io/pytauri/latest/usage/tutorial/build-standalone/>

1. Use `download-py` to download `python-build-standalone` (only needed for the first build).
    You can modify `download-py` to customize the required Python version.
2. Use `build` to build the app.
    If you want to use `Cython` to protect your python source code,
    set the environment variable `USE_CYTHON=1` before running `build`.
