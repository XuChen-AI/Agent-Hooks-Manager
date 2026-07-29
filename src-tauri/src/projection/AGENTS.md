<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# projection

## Purpose
Projection layer - compiles a canonical `Hook` into each target agent's native `NativeHook` shape, emitting a loss-report (`ProjectionGap`) for anything an agent cannot represent rather than silently dropping it.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `project()` (Hook -> NativeHook per adapter, with gap on unsupported event) and `deploy()` (project + collect hooks for one adapter, returning all gaps) |

## Subdirectories
No subdirectories.

## For AI Agents
`project()` rejects hooks whose event the adapter doesn't support and returns a `ProjectionGap`. `deploy()` filters hooks by `target_agents`, projects each, and accumulates gaps - the actual `adapter.write_hooks()` call is still a TODO. Per-agent matcher-syntax translation (Grok regex vs Claude permissions-style vs Codex) is also TODO; matchers are currently passed through verbatim.

<!-- deepinit:generated end -->

## Manual Notes
