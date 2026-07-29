<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# src-tauri

## Purpose
Tauri 2 Rust backend crate for agent-hooks-manager. Defines the native app shell, build config, and cargo manifest; the actual backend modules live under `src/`.

## Key Files
| File | Description |
|------|-------------|
| `Cargo.toml` | Rust crate manifest; Tauri 2 + rusqlite (bundled) + toml_edit + serde + tokio + dirs; lib name `agent_hooks_manager_lib` (staticlib/cdylib/rlib) |
| `tauri.conf.json` | Tauri 2 config; productName, 1100x720 window, `frontendDist` `../dist`, `devUrl` `http://localhost:1420`, `beforeDevCommand` `npm run dev` |
| `build.rs` | Tauri build script; calls `tauri_build::build()` |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | Backend Rust source: adapters, commands, db, model, projection, sync, trust (see `src/AGENTS.md`) |

## For AI Agents
Crate name `agent_hooks_manager_lib` (lib + bin). `rusqlite` is bundled (no external SQLite needed); `toml_edit` preserves comments on TOML writes. `tauri.conf.json` wires the frontend dev/build commands - run the whole app with `npm run tauri dev` from the project root, not from here.

<!-- deepinit:generated end -->

## Manual Notes
