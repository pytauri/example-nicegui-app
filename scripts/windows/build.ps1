Set-Location (Resolve-Path "$PSScriptRoot\..\..")

$PROJECT_NAME = "nicegui-app"

$env:PYTAURI_STANDALONE = "1"
$env:PYO3_PYTHON = (Resolve-Path -LiteralPath "pyembed\python\python.exe").Path

uv.exe pip install `
    --exact `
    --compile-bytecode `
    --python="$env:PYO3_PYTHON" `
    --reinstall-package="$PROJECT_NAME" `
    .

cargo -- tauri build --config="tauri.bundle.json" -- --profile bundle-release
