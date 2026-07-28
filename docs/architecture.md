# agent-hooks-manager 架构 / Architecture

## 设计原则

抄 cc-switch 的工程骨架（Tauri2 + Rust + SQLite SSOT + 双向同步 + 原子写），把「provider 切换」换成「hooks 投影」，加一层 `ToolAdapter` trait 让每个 agent 可插拔。

## 分层架构 / Layered Architecture

```
┌──────────────────────────────────────────────────────────┐
│                    GUI (React + Tauri2)                   │
│  Hooks 列表 │ Agent 状态 │ 残留扫描 │ 性能日志 │ 托盘     │
└────────────────────────┬─────────────────────────────────┘
                         │ Tauri IPC (commands)
┌────────────────────────▼─────────────────────────────────┐
│              投影编译层 Projection                         │
│   canonical Hook ──► Claude  ~/.claude/settings.json      │
│                  ──► Codex   ~/.codex/hooks.json          │
│                  ──► Grok    ~/.grok/hooks/*.json         │
│   (+ loss-report: 不兼容项告警，不静默失败)                 │
│   双向 backfill │ 原子写 (temp+rename) │ toml_edit 保注释 │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│            SSOT 模型层 (SQLite)                            │
│   Hook{ id, name, event, matcher, type, command/url,      │
│         target_agents[], enabled, scope, source }         │
│   事件超集: SessionStart|UserPromptSubmit|PreToolUse|      │
│            PostToolUse|Stop|Notification|SessionEnd…       │
└────────────────────────┬─────────────────────────────────┘
                         │
┌────────────────────────▼─────────────────────────────────┐
│        Adapter 层 (Rust ToolAdapter trait)                 │
│  ┌────────────┐ ┌──────────┐ ┌────────────┐ ┌─────────┐   │
│  │ClaudeCode  │ │  Codex   │ │ GrokBuild  │ │ Gemini… │   │
│  │settings.json│ │hooks.json│ │hooks/*.json│ │         │   │
│  └────────────┘ └──────────┘ └────────────┘ └─────────┘   │
└──────────────────────────────────────────────────────────┘
        ▲ 部署后各 agent 的信任流（不可绕过）：
        │  Codex → /hooks 手动批准 │ Claude → 工作区信任 │ Grok → /hooks-trust
```

## 各层职责 / Layer Responsibilities

### 1. Adapter 层
每个 agent 一个文件，实现 `ToolAdapter` trait：detect 安装 / config 路径 / 配置格式 / 读 hooks / 写 hooks / 事件词表 / 信任模型 / 是否需重启。**加 agent = 加一个文件，不动核心。** 参考 weykon/agent-hooks。

### 2. SSOT 模型层
canonical Hook 实体存 SQLite，事件用超集枚举，每个 hook 带 `target_agents[]`（部署到哪些 agent）。单一数据源，所有 GUI 操作改这里。

### 3. 投影编译层
canonical → 各 agent 原生 JSON/TOML。遇不兼容项（某 agent 不支持的事件/matcher）生成 loss-report 告警，不静默失败（参考 Hookbridge）。TOML 用 `toml_edit` 保注释，JSON 用结构化合并。双向 backfill：部署时写 live 文件，读取时从 live 回读，检测用户手改。

### 4. 信任/安全层（最易踩坑）
- Codex 写完 hooks 不会自动生效，必须用户在 Codex 里 `/hooks` 批准（哈希信任）——UI 显式提示
- Claude 工作区信任；Grok 项目 `/hooks-trust`
- GUI 为每个 agent 显示信任状态
- Codex Windows hooks 支持需本机验证（曾暂时禁用）

### 5. GUI 层
Tauri2 + React：hooks 跨 agent 聚合列表、表单+JSON 双编辑、残留扫描器、性能日志（p95/循环检测）、预设市场、系统托盘。

### 6. 运行时桥接（可选，差异化）
所有 hook 路由过一个桥接脚本，事件归一为 JSONL，得到**跨 agent 统一执行日志**（cc-switch 都没有）。

## 数据流 / Data Flow

```
GUI 编辑 hook → 存 SQLite → 投影层翻译成 N 份原生配置写盘
  → 提示去各 agent 完成信任 → hook 生效 → 桥接脚本回传执行日志 → GUI 展示
```

## 各 agent hooks 协议对照 / Agent Protocol Matrix

| Agent | 配置位置 | 格式 | 事件 | 信任 | 重启 |
|---|---|---|---|---|---|
| Claude Code | `~/.claude/settings.json` | JSON | PreToolUse/PostToolUse/Stop/UserPromptSubmit/Notification/SessionStart/SessionEnd/PreCompact/SubagentStop | 工作区信任 | 否（热切换） |
| Codex | `~/.codex/hooks.json` | JSON | SessionStart/PreToolUse/PostToolUse/UserPromptSubmit/Stop（GA 后 10 个） | 哈希信任，`/hooks` 手动批准 | 是 |
| Grok Build | `~/.grok/hooks/*.json`（+ 读 Claude settings.json） | JSON | SessionStart/UserPromptSubmit/PreToolUse/PostToolUse/Stop/Notification/SessionEnd | 全局免信任；项目 `/hooks-trust` | 否 |
| Gemini CLI | `~/.gemini/hooks.json` | JSON | turn_complete/user_prompt_submit | — | 是 |
| Cursor | `~/.cursor/hooks.json` | JSON | stop/preToolUse/postToolUse/beforeSubmitPrompt… | — | 是 |

**关键洞察**：Grok Build 直接读 `~/.claude/settings.json`，事件名/schema/matcher 与 Claude 几乎一致——三者天然可归一化。

## 目录结构 / Project Structure

```
agent-hooks-manager/
├── docs/                      # 文档
│   ├── architecture.md        # 本文件
│   ├── adapters.md            # 如何加一个 agent adapter
│   └── roadmap.md             # 路线图
├── src-tauri/                 # Rust 后端
│   └── src/
│       ├── adapters/          # ToolAdapter trait + 各 agent 实现
│       ├── model/             # canonical Hook 模型
│       ├── projection/        # 投影编译 + loss-report
│       ├── sync/              # 双向 backfill + 原子写
│       ├── trust/             # 信任/安全
│       ├── db/                # SQLite DAO
│       └── commands/          # Tauri IPC 命令层
├── src/                       # React 前端
└── resources/                 # 预设 hooks 市场
```
