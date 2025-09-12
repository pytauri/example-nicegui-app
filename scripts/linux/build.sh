#!/bin/bash

set -e

cd "$(dirname "$0")/../.."

PROJECT_NAME="nicegui-app"
PYLIB_DIR="$(realpath pyembed/python/lib)"

export PYTAURI_STANDALONE="1"
export PYO3_PYTHON="$(realpath pyembed/python/bin/python3)"
export RUSTFLAGS=" \
    -C link-arg=-Wl,-rpath,\$ORIGIN/../lib/$PROJECT_NAME/lib \
    -L $PYLIB_DIR"

uv pip install \
    --exact \
    --compile-bytecode \
    --python="$PYO3_PYTHON" \
    --reinstall-package="$PROJECT_NAME" \
    .

cargo tauri build --config="tauri.bundle.json" -- --profile bundle-release
