<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# src

## Purpose
React 19 + Tailwind frontend for the Tauri desktop app. Scaffold only - renders a minimal shell that calls Tauri IPC commands to list detected agents and scan for third-party residue.

## Key Files
| File | Description |
|------|-------------|
| `App.tsx` | Frontend shell scaffold; invokes `list_agents` and `scan_residue` Tauri commands and renders detected agents + residue |
| `main.tsx` | React entry point; mounts `App` into `#root` under `React.StrictMode` |
| `styles.css` | Tailwind CSS entry (`@tailwind base/components/utilities`) |

## Subdirectories
No subdirectories.

## For AI Agents
Scaffold only. `App.tsx` is the UI root and the place to build out the planned pages: Hooks list, Agent status, Residue scan, Performance log, Presets. It talks to the backend via `@tauri-apps/api/core` `invoke` against commands registered in `src-tauri/src/commands/mod.rs`.

<!-- deepinit:generated end -->

## Manual Notes
