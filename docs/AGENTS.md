<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# docs

## Purpose
Design documentation for agent-hooks-manager (bilingual Chinese/English). Covers the layered architecture, the protocol for adding agent adapters, and the project roadmap.

## Key Files
| File | Description |
|------|-------------|
| `architecture.md` | Canonical design: layered architecture (GUI / Projection / SSOT / Adapter / Trust), data flow, agent protocol matrix, project structure |
| `adapters.md` | How to add a new agent adapter - 5-step guide, the `ToolAdapter` trait reference, and known caveats for the MVP adapters |
| `roadmap.md` | Roadmap with P0 (MVP), P1 (residue cleanup + observability), P2 (platform) priorities and explicit non-goals |

## Subdirectories
No subdirectories.

## For AI Agents
`architecture.md` is the authoritative design reference (read it before touching backend layers). `adapters.md` documents the extension contract mirrored by `src-tauri/src/adapters/`. `roadmap.md` tracks what is in scope per phase.

<!-- deepinit:generated end -->

## Manual Notes
