// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{convert::Infallible, env::var, error::Error, path::PathBuf};

use pyo3::wrap_pymodule;
use pytauri::standalone::{
    dunce::simplified, PythonInterpreterBuilder, PythonInterpreterEnv, PythonScript,
};
use tauri::{Builder, Manager as _};

use nicegui_app_lib::{ext_mod, tauri_generate_context};

fn main() -> Result<Infallible, Box<dyn Error>> {
    let py_env = if cfg!(dev) {
        // `cfg(dev)` is set by `tauri-build` in `build.rs`, which means running with `tauri dev`,
        // see: <https://github.com/tauri-apps/tauri/pull/8937>.

        let venv_dir = var("VIRTUAL_ENV").map_err(|err| {
            format!(
                "The app is running in tauri dev mode, \
                please activate the python virtual environment first \
                or set the `VIRTUAL_ENV` environment variable: {err}",
            )
        })?;
        PythonInterpreterEnv::Venv(PathBuf::from(venv_dir).into())
    } else {
        // embedded Python, i.e., bundle mode with `tauri build`.

        // Actually, we don't use this app, we just use it to get the resource directory
        let sample_app = Builder::default()
            .build(tauri_generate_context())
            .map_err(|err| format!("failed to build sample app: {err}"))?;
        let resource_dir = sample_app
            .path()
            .resource_dir()
            .map_err(|err| format!("failed to get resource dir: {err}"))?;

        // 👉 Remove the UNC prefix `\\?\`, Python ecosystems don't like it.
        let resource_dir = simplified(&resource_dir).to_owned();

        // 👉 When bundled as a standalone App, we will put python in the resource directory
        PythonInterpreterEnv::Standalone(resource_dir.into())
    };

    // 👉 Equivalent to `python -m nicegui_app`,
    // i.e, run the `nicegui-app/python/nicegui_app/__main__.py`
    let py_script = PythonScript::Module("nicegui_app".into());

    // 👉 `ext_mod` is your extension module, we export it from memory,
    // so you don't need to compile it into a binary file (.pyd/.so).
    let builder =
        PythonInterpreterBuilder::new(py_env, py_script, |py| wrap_pymodule!(ext_mod)(py));
    let interpreter = builder.build()?;

    let exit_code = interpreter.run();
    std::process::exit(exit_code);
}
