<!-- deepinit:generated start -->
<!-- Parent:  -->

# agent-hooks-manager

## Purpose
Desktop app (Tauri 2 + Rust + React 19) that unifies hooks management across AI coding agents - Claude Code, Codex, Grok Build, and more - from one GUI. Author a canonical hook once, project it into each agent's native config, detect and clean up third-party residue, and observe per-hook execution. Pre-alpha scaffold; not runnable yet.

## Key Files
| File | Description |
|------|-------------|
| `package.json` | npm manifest; Tauri 2 + React 19 + TypeScript + Vite + Tailwind; scripts: dev, build, preview, tauri |
| `index.html` | Vite HTML entry; mounts `#root` and loads `/src/main.tsx` |
| `README.md` | English project README - what it is, FAQ, how-it-works mermaid diagram, status, tech stack |
| `README.zh-CN.md` | Chinese translation of the project README |
| `LICENSE` | MIT license text |
| `.gitignore` | Ignores node_modules, build output (dist/, src-tauri/target/, src-tauri/gen/), Tauri WixTools, editor/OS files, local DB/logs (*.db, *.db-journal) |

## Subdirectories
| Directory | Purpose |
|-----------|---------|
| `src/` | React 19 + Tailwind frontend (scaffold) (see `src/AGENTS.md`) |
| `src-tauri/` | Tauri 2 Rust backend crate (see `src-tauri/AGENTS.md`) |
| `docs/` | Design docs: architecture, adapters, roadmap (see `docs/AGENTS.md`) |
| `resources/` | Preset hooks marketplace (see `resources/AGENTS.md`) |
| `assets/` | Static image assets (see `assets/AGENTS.md`) |

## For AI Agents
Pre-alpha scaffold, not runnable yet. Rust backend lives in `src-tauri/`, React frontend in `src/`. Dev: `npm run dev` (Vite on :1420) + `npm run tauri dev`; build: `npm run build`. SQLite is the SSOT for hooks; the `ToolAdapter` trait (`src-tauri/src/adapters/trait.rs`) is the core abstraction - add an agent by adding one file. Canonical design: `docs/architecture.md`; roadmap: `docs/roadmap.md`.

<!-- deepinit:generated end -->

## Manual Notes
