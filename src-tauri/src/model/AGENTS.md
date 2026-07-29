<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# model

## Purpose
Canonical, agent-agnostic hook model - the single source of truth stored in SQLite. Defines the superset of events/types/scopes and the `Source` tag that drives residue detection.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | Canonical `Hook` struct plus `Event`, `HookType`, `Scope`, and `Source` enums (all `serde`-serializable) |

## Subdirectories
No subdirectories.

## For AI Agents
`Hook` is agent-agnostic; the projection layer translates it per adapter. `Event` is a superset across agents (adapters expose their supported subset via `supported_events()`). `Source` (`User` / `ThirdParty { tool_name }` / `Preset { preset_id }`) is what the residue scanner keys on to flag leftover third-party hooks. The SQLite schema in `db/mod.rs` mirrors this struct.

<!-- deepinit:generated end -->

## Manual Notes
