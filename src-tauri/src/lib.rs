//! agent-hooks-manager backend entry point.
//!
//! Scaffold only - not runnable yet. Wires the Tauri command layer to the
//! adapter / projection / sync / trust / db modules defined below.

mod adapters;
mod commands;
mod db;
mod model;
mod projection;
mod sync;
mod trust;

// Re-export the trait under a non-reserved name.
pub use adapters::ToolAdapter;

use std::sync::Mutex;

/// Shared application state injected into Tauri.
pub struct AppState {
    pub db: Mutex<db::Database>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            db: Mutex::new(db::Database::open_in_memory().expect("open db")),
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_hooks,
            commands::upsert_hook,
            commands::delete_hook,
            commands::deploy,
            commands::scan_residue,
            commands::list_agents,
        ])
        .run(tauri::generate_context!())
        .expect("error while running agent-hooks-manager");
}
