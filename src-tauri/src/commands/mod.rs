//! Tauri IPC command layer. Thin wrappers over the service modules.

use crate::model::Hook;

#[tauri::command]
pub fn list_hooks(_state: tauri::State<'_, crate::AppState>) -> Vec<Hook> {
    // TODO: db.list_hooks()
    Vec::new()
}

#[tauri::command]
pub fn upsert_hook(_state: tauri::State<'_, crate::AppState>, _hook: Hook) -> bool {
    // TODO: db.upsert(hook)
    true
}

#[tauri::command]
pub fn delete_hook(_state: tauri::State<'_, crate::AppState>, _id: String) -> bool {
    // TODO
    true
}

/// Deploy all hooks to all target agents. Returns projection gaps (loss-report).
#[tauri::command]
pub fn deploy(_state: tauri::State<'_, crate::AppState>) -> Vec<crate::projection::ProjectionGap> {
    // TODO: for each adapter, projection::deploy(hooks, adapter); collect gaps.
    Vec::new()
}

/// Scan for third-party residue (uninstalled tools' leftover hooks).
#[tauri::command]
pub fn scan_residue(_state: tauri::State<'_, crate::AppState>) -> Vec<String> {
    crate::trust::detect_residue()
}

#[tauri::command]
pub fn list_agents() -> Vec<String> {
    crate::adapters::all_adapters()
        .iter()
        .map(|a| a.display_name().to_string())
        .collect()
}
