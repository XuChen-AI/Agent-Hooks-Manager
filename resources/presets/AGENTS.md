<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# presets

## Purpose
Individual preset hook JSON files for the marketplace. Each file is a one-click installable hook matching the canonical `Hook` model, ready to be projected to its `target_agents`.

## Key Files
| File | Description |
|------|-------------|
| `rm-rf-guard.json` | Preset blocking dangerous `rm -rf` shell commands (PreToolUse, matcher `Bash`, targets claude-code/codex/grok-build) |

## Subdirectories
No subdirectories.

## For AI Agents
Each JSON file conforms to the canonical `Hook` model (`src-tauri/src/model/mod.rs`) and carries a `target_agents` list plus a `source: { "Preset": { "preset_id": ... } }` tag. `resources/README.md` lists additional planned presets (auto-format, audit-log, stop-notify, secret-scan) not yet present here.

<!-- deepinit:generated end -->

## Manual Notes
