# PyTauri + NiceGui

> Make sure you have already installed `rust`, `Python`, `uv`, Tauri Prerequisites.
>
> You can find that in tutorial section in PyTauri Documentation.

Clone this repository and `cd` into it first.

## Standalone

```bash
# activate virtual environment
uv venv --python-preference only-system
source .venv/bin/activate

# install the example package
export PYTAURI_STANDALONE=1
# or powershell: $env:PYTAURI_STANDALONE=1
uv pip install --reinstall -e .

cargo tauri dev --no-dev-server-wait
```

## Python

```bash
# activate virtual environment
uv venv --python-preference only-system
source .venv/bin/activate
# or powershell: .venv\Scripts\Activate.ps1

# install the example package
# (need some time to compile rust code,
#  you can pass `--verbose` to see the progress)
uv pip install --reinstall -e .

# run the example
python -m nicegui_app
```
