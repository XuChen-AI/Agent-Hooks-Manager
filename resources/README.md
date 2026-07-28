# Preset Hooks Marketplace / 预设 Hook 市场

One-click installable hook presets for agent-hooks-manager. Each preset is a JSON
file matching the canonical `Hook` model (see `src-tauri/src/model/mod.rs`) with a
`target_agents` list. agent-hooks-manager installs them via the GUI marketplace.

## Layout

```
resources/presets/
├── rm-rf-guard.json      # block dangerous shell commands (PreToolUse)
├── auto-format.json      # run formatter after edits (PostToolUse)
├── audit-log.json        # log every tool call (PreToolUse, async)
├── stop-notify.json      # notify on session end (Stop)
└── secret-scan.json      # block writes to .env / secrets (PreToolUse)
```

## Example preset

```json
{
  "name": "rm -rf Guard",
  "event": "PreToolUse",
  "matcher": "Bash",
  "hook_type": "Command",
  "command": "bash ~/.agent-hooks-manager/scripts/rm-rf-guard.sh",
  "timeout": 5,
  "enabled": true,
  "scope": "Global",
  "target_agents": ["claude-code", "codex", "grok-build"],
  "source": { "Preset": { "preset_id": "rm-rf-guard" } }
}
```
