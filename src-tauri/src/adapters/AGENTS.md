<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# adapters

## Purpose
Adapter layer - one `ToolAdapter` implementation per supported AI coding agent. Each adapter knows how to detect, read, write, and unregister that agent's native hooks. Adding an agent means adding one file here and registering it in `all_adapters()`.

## Key Files
| File | Description |
|------|-------------|
| `trait.rs` | Core `ToolAdapter` trait plus `ConfigFormat`, `TrustModel`, `NativeHook`, and `AdapterError` |
| `mod.rs` | Module root; re-exports the trait and `all_adapters()` returning Claude/Codex/Grok (Gemini/Cursor TODO) |
| `claude.rs` | Claude Code adapter; `~/.claude/settings.json` (JSON), workspace trust, hot-swap no restart (methods mostly TODO) |
| `codex.rs` | Codex adapter; `~/.codex/hooks.json` (JSON), hash trust requiring `/hooks` approval, restart required (TODO) |
| `grok.rs` | Grok Build adapter; `~/.grok/hooks/*.json` (JSON, also reads Claude settings.json), project trust (TODO) |

## Subdirectories
No subdirectories.

## For AI Agents
One `ToolAdapter` impl per agent. Note `trait.rs` is declared as `mod r#trait` in `mod.rs` (and re-exported as `ToolAdapter`) because `trait` is a reserved word. All three adapter impls are stubs - `is_installed`, `read_hooks`, `write_hooks`, `unregister_all` return defaults/TODOs. Extension guide: `docs/adapters.md`. Grok reads `~/.claude/settings.json` for Claude compat, so the projection layer must avoid double-writing.

<!-- deepinit:generated end -->

## Manual Notes
