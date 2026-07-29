<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# trust

## Purpose
Trust and safety layer. Surfaces each agent's non-bypassable post-deploy trust gate and detects third-party residue (leftover hooks from uninstalled tools) for cleanup.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `post_deploy_instruction()` returns the manual step per `TrustModel`; `detect_residue()` flags third-party leftover hooks (TODO) |

## Subdirectories
No subdirectories.

## For AI Agents
`post_deploy_instruction()` maps each `TrustModel` to the exact manual action the user must take (workspace trust for Claude, `/hooks` approval for Codex, `/hooks-trust` for Grok) - the GUI must surface these, it cannot promise one-click. `detect_residue()` is the residue scanner's core (called by the `scan_residue` Tauri command) but is currently a stub returning an empty list.

<!-- deepinit:generated end -->

## Manual Notes
