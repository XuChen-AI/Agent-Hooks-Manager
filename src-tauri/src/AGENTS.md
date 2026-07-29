<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# src

## Purpose
Backend Rust source root. Wires the Tauri app together and houses the module tree: adapters, commands, db, model, projection, sync, trust. Scaffold only - modules compile but most logic is TODO stubs.

## Key Files
| File | Description |
|------|-------------|
| `lib.rs` | Library entry; declares all modules, re-exports `ToolAdapter`, defines `AppState` (Mutex<Database>), and `run()` builds the Tauri app + registers IPC commands |
| `main.rs` | Binary entry; calls `agent_hooks_manager_lib::run()` and hides the console window in release builds |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `adapters/` | `ToolAdapter` trait + per-agent implementations (see `adapters/AGENTS.md`) |
| `model/` | Canonical agent-agnostic `Hook` model (see `model/AGENTS.md`) |
| `projection/` | Canonical -> native config compilation with loss-report (see `projection/AGENTS.md`) |
| `sync/` | Atomic writes + bidirectional backfill (see `sync/AGENTS.md`) |
| `trust/` | Trust/safety gates + residue detection (see `trust/AGENTS.md`) |
| `db/` | SQLite DAO, SSOT persistence (see `db/AGENTS.md`) |
| `commands/` | Tauri IPC command layer (see `commands/AGENTS.md`) |

## For AI Agents
`lib.rs` is the wiring point: it registers the IPC handlers (`list_hooks`, `upsert_hook`, `delete_hook`, `deploy`, `scan_residue`, `list_agents`) and injects an in-memory `AppState`. The module declaration order here is the canonical list of backend layers - match it when navigating. Scaffold only; not runnable end-to-end yet.

<!-- deepinit:generated end -->

## Manual Notes
