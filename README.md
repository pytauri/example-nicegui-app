# PyTauri + NiceGui

> Make sure you have already installed `rust`, `python`, `uv` and Tauri Prerequisites.
>
> You can find that in tutorial section in PyTauri Documentation.
>
> Also, you need [`tauri-cli`](https://tauri.app/reference/cli/), you can install it by `cargo install tauri-cli`.

Clone this repository and `cd` into it first.

## Development

```bash
# create and activate virtual environment
uv venv --python-preference only-system
source .venv/bin/activate
# or powershell: .venv\Scripts\activate

# install the example package
uv sync

# run the app
cargo tauri dev --no-dev-server-wait
```

## Build

See [`scripts/README.md`](scripts/README.md).
