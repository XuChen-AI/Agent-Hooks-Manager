<p align="center">
  <img src="./assets/logo.png" alt="agent-hooks-manager logo" width="256">
</p>

# agent-hooks-manager

> Unified hooks manager for AI coding agents - manage, deploy, audit, and clean up hooks across Claude Code, Codex, Grok Build, Gemini CLI, Cursor, and more.

**Keywords / 搜索关键词:** agent hooks manager · claude code hooks manager · codex hooks manager · grok build hooks · manage hooks across agents · unified hooks management · clean up third-party hooks · remove hooks left by uninstalled tools · hooks out of control · too many hooks slow · claude code hooks GUI

[中文文档](./README.zh-CN.md)

## What it is

`agent-hooks-manager` is a desktop app to **manage hooks across multiple AI coding agents from one place**. Every agent - Claude Code, Codex, Grok Build, Gemini CLI, Cursor - has its own hook system, stored in a different file, in a different format (JSON/TOML), with a different trust flow. This app gives you one GUI to: author a hook once and deploy it to every agent; detect and clean up hooks left behind by uninstalled third-party tools; and see per-hook execution logs, latency, and loops.

## FAQ - the questions people ask

- **How do I manage hooks across Claude Code and Codex?** Author one canonical hook here; it's projected into each agent's native config format.
- **How to remove hooks left by an uninstalled plugin or tool?** The residue scanner finds third-party hooks (e.g. entries in `~/.claude/settings.json` pointing at a deleted app's scripts) and removes them across all scopes (user / project / local).
- **Why are my Claude Code hooks slow, or looping forever?** Per-hook execution log + p95 latency + loop detection pinpoints the culprit. Hooks are invisible by default - see [anthropics/claude-code#44732](https://github.com/anthropics/claude-code/issues/44732).
- **How to clean up third-party hooks in settings.json?** Scan -> review -> one-click remove, across scopes.
- **Is there a GUI for Claude Code hooks?** Yes - a Tauri 2 desktop app, no more hand-editing JSON.
- **How to deploy the same hook to Claude Code, Codex, and Grok Build at once?** Check the target agents on the hook; deploy writes each native file and tells you the per-agent trust step.
- **A tool I uninstalled left hooks in my Claude Code config - how to clean it?** That's the residue scanner's exact job.

## Why (real pain users hit)

- **Hooks are invisible.** No logs. Can't tell which hook is slow or looping until the session dies. ([#44732](https://github.com/anthropics/claude-code/issues/44732))
- **Third-party tools leave hooks behind.** A plugin writes hooks into `~/.claude/settings.json` at install time; you uninstall the app, the hooks stay. ([#48100](https://github.com/anthropics/claude-code/issues/48100))
- **Configs multiply.** Hooks progressively duplicate until you have hundreds. ([#3523](https://github.com/anthropics/claude-code/issues/3523))
- **Death by a thousand hooks.** 100+ hooks add seconds of overhead per tool call. ([dawidgac](https://dawidgac.com/en/blog/claude-code-hooks-optimization))
- **No management UI, no hot-reload.** Everything is hand-edited JSON. ([#36121](https://github.com/anthropics/claude-code/issues/36121))

**No existing tool unifies hooks across agents with a GUI, residue cleanup, and observability - this fills that gap.**

## How it works

```mermaid
flowchart TD
    GUI["🖥️ GUI<br/>React + Tauri 2<br/>Hooks list · Residue scan · Perf log · Presets"]
    PROJ["📐 Projection layer<br/>canonical Hook → native config (+ loss-report)"]
    DB[("💾 SQLite SSOT<br/>Hook{event, matcher, type,<br/>target_agents[], source}")]
    ADAPT["🔌 Adapter layer<br/>Rust ToolAdapter trait · one file per agent"]

    GUI -->|edit / deploy| PROJ
    PROJ --> DB
    DB --> ADAPT

    ADAPT --> CC["Claude Code<br/>~/.claude/settings.json"]
    ADAPT --> CX["Codex<br/>~/.codex/hooks.json"]
    ADAPT --> GB["Grok Build<br/>~/.grok/hooks/*.json"]

    CC -.->|workspace trust| T["🔒 Trust flow<br/>non-bypassable, surfaced in GUI"]
    CX -.->|/hooks approve| T
    GB -.->|/hooks-trust| T
```

Full design: [`docs/architecture.md`](./docs/architecture.md). Add an agent: [`docs/adapters.md`](./docs/adapters.md).

## Status

Pre-alpha scaffold. Not runnable yet. See [`docs/roadmap.md`](./docs/roadmap.md).

## Tech stack

Tauri 2 · Rust · React 19 · TypeScript · SQLite · TailwindCSS

## License

MIT
