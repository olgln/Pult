//! Window bootstrap. Keep this file free of logic: everything testable lives in other
//! modules or in `pult-core`.

mod commands;
#[cfg(test)]
mod config_tests;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![commands::app_version])
        .run(tauri::generate_context!())
        .expect("error while running Pult");
}
