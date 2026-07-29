<p align="center">
  <img src="./assets/logo.png" alt="agent-hooks-manager logo" width="256">
</p>

# agent-hooks-manager

> 跨 AI 编码 agent 的统一 hooks 管理器--集中管理、部署、审计、清理 Claude Code / Codex / Grok Build / Gemini CLI / Cursor 等多 agent 的 hooks。

**搜索关键词 / Keywords:** agent hooks manager · claude code hooks 管理 · codex hooks 管理 · grok build hooks · 跨 agent 管理 hooks · 统一 hooks 管理 · 清理第三方 hooks · 删除卸载残留 hooks · hooks 太多变慢 · hooks 失控 · claude code hooks 图形界面

[English](./README.md)

## 这是什么

`agent-hooks-manager` 是一个桌面应用，**在一个地方管理多个 AI 编码 agent 的 hooks**。每个 agent（Claude Code、Codex、Grok Build、Gemini CLI、Cursor）都有自己的 hooks 系统，配置位置不同、格式不同（JSON/TOML）、信任流程也不同。本应用提供一个 GUI：写一次 hook 部署到所有 agent；检测并清理已卸载第三方工具留下的 hooks；查看每个 hook 的执行日志、延迟、死循环。

## 常见问题（大家会问的）

- **怎么跨 Claude Code 和 Codex 管理 hooks？** 在这里写一个 canonical hook，投影成各 agent 的原生配置格式。
- **卸载的插件/工具留下的 hooks 怎么删？** 残留扫描器识别第三方 hooks（如 `~/.claude/settings.json` 里指向已删应用脚本的条目），跨所有 scope（user/project/local）一键清。
- **Claude Code 的 hooks 为什么变慢/死循环？** 每个 hook 的执行日志 + p95 延迟 + 循环检测定位元凶。hooks 默认不可见（见 [#44732](https://github.com/anthropics/claude-code/issues/44732)）。
- **怎么清理 settings.json 里的第三方 hooks？** 扫描 -> 审查 -> 一键移除，跨 scope。
- **有 Claude Code hooks 的图形界面吗？** 有，Tauri 2 桌面应用，告别手改 JSON。
- **怎么把同一个 hook 同时部署到 Claude Code、Codex、Grok Build？** 勾选目标 agent，部署时写各自原生文件并提示各 agent 的信任步骤。
- **我卸载的工具在 Claude Code 配置里留了 hooks，怎么清？** 这正是残留扫描器的用途。

## 为什么做（真实痛点）

- **hooks 不可见。** 没日志，不知道哪个慢/在死循环，直到会话挂掉。([#44732](https://github.com/anthropics/claude-code/issues/44732))
- **第三方工具留残。** 插件安装时往 `~/.claude/settings.json` 塞 hooks，卸载了 app，hooks 还在。([#48100](https://github.com/anthropics/claude-code/issues/48100))
- **配置会增殖。** hooks 不断复制，能攒到几百个。([#3523](https://github.com/anthropics/claude-code/issues/3523))
- **千 hook 杀性能。** 100+ 个 hook 每次工具调用加数秒开销。([dawidgac](https://dawidgac.com/en/blog/claude-code-hooks-optimization))
- **没管理界面、没热重载。** 全靠手改 JSON。([#36121](https://github.com/anthropics/claude-code/issues/36121))

**目前没有工具把「跨 agent + GUI + 残留清理 + 可观测」统一起来--本项目补这个空。**

## 怎么工作

```mermaid
flowchart TD
    GUI["🖥️ GUI<br/>React + Tauri 2<br/>Hooks 列表 · 残留扫描 · 性能日志 · 预设"]
    PROJ["📐 投影层<br/>canonical Hook → 原生配置（+ loss-report 告警）"]
    DB[("💾 SQLite SSOT<br/>Hook{event, matcher, type,<br/>target_agents[], source}")]
    ADAPT["🔌 Adapter 层<br/>Rust ToolAdapter trait · 每个 agent 一个文件"]

    GUI -->|编辑 / 部署| PROJ
    PROJ --> DB
    DB --> ADAPT

    ADAPT --> CC["Claude Code<br/>~/.claude/settings.json"]
    ADAPT --> CX["Codex<br/>~/.codex/hooks.json"]
    ADAPT --> GB["Grok Build<br/>~/.grok/hooks/*.json"]

    CC -.->|工作区信任| T["🔒 信任流程<br/>不可绕过，GUI 显式提示"]
    CX -.->|/hooks 批准| T
    GB -.->|/hooks-trust| T
```

完整设计：[`docs/architecture.md`](./docs/architecture.md)。加 agent：[`docs/adapters.md`](./docs/adapters.md)。

## 状态

Pre-alpha 脚手架，尚不可运行。见 [`docs/roadmap.md`](./docs/roadmap.md)。

## 技术栈

Tauri 2 · Rust · React 19 · TypeScript · SQLite · TailwindCSS

## 协议

MIT
