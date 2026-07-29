<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# resources

## Purpose
Bundled resources for agent-hooks-manager. Currently hosts the preset hooks marketplace - one-click installable hook presets matching the canonical `Hook` model.

## Key Files
| File | Description |
|------|-------------|
| `README.md` | Describes the preset marketplace layout, lists planned presets, and shows the example preset JSON schema |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `presets/` | Individual preset hook JSON files (see `presets/AGENTS.md`) |

## For AI Agents
Preset marketplace root. Each preset is a JSON file conforming to the canonical `Hook` model (`src-tauri/src/model/mod.rs`) with a `target_agents` list and a `Preset` source. See `README.md` for the intended layout.

<!-- deepinit:generated end -->

## Manual Notes
