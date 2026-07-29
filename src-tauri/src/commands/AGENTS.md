<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# commands

## Purpose
Tauri IPC command layer. Thin `#[tauri::command]` wrappers exposing the backend service modules to the React frontend.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Tauri commands: `list_hooks`, `upsert_hook`, `delete_hook`, `deploy`, `scan_residue`, `list_agents` (thin wrappers, mostly TODO stubs) |

## Subdirectories
No subdirectories.

## For AI Agents
These commands are registered in `src-tauri/src/lib.rs` via `generate_handler!` and invoked from the frontend with `invoke(...)`. `list_agents` and `scan_residue` are the only ones currently wired to real logic (delegating to `adapters::all_adapters()` and `trust::detect_residue()`); the hook CRUD and `deploy` commands are TODO stubs.

<!-- deepinit:generated end -->

## Manual Notes
