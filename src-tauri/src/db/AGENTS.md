<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# db

## Purpose
SQLite DAO layer - the single source of truth for all canonical hooks. Owns the connection and schema migrations mirroring `model::Hook`.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `Database` struct with `open_in_memory()` and `migrate()` creating the `hooks` table; CRUD methods (list/upsert/delete/find_by_source) are TODO |

## Subdirectories
No subdirectories.

## For AI Agents
Schema mirrors `model::Hook` (id, name, event, matcher, hook_type, command/url, timeout, env, enabled, scope, target_agents, source). Currently opens in-memory only (wired in `lib.rs` `AppState`) and exposes no CRUD yet - those methods are the next thing to implement. `rusqlite` is bundled so no external SQLite is required.

<!-- deepinit:generated end -->

## Manual Notes
