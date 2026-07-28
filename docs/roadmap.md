# 路线图 / Roadmap

## P0 — MVP（能跑通 3 个 agent）

- [ ] `ToolAdapter` trait + Claude/Codex/Grok 三个 adapter
- [ ] canonical Hook 模型 + SQLite SSOT
- [ ] 投影层：canonical → 三个 agent 原生 JSON
- [ ] 基础 GUI：hooks 列表 / 启停 / 编辑 / 部署
- [ ] 各 agent 信任状态提示
- [ ] 5~8 个内置预设 hook

## P1 — 残留清理 + 可观测（差异化卖点）

- [ ] 残留扫描器：识别第三方留下的 hooks/MCP（source = ThirdParty）
- [ ] 跨 scope 一键清理（user/project/local，参考 #7936 的坑）
- [ ] 桥接脚本：统一执行日志 JSONL
- [ ] p95 性能分析 + 循环检测（参考 #44732）
- [ ] 双向 backfill（检测用户手改）
- [ ] 加 Gemini / Cursor adapter

## P2 — 平台化

- [ ] Hook 预设市场（GitHub repo 一键装）
- [ ] 云同步（Dropbox/OneDrive/WebDAV，抄 cc-switch）
- [ ] 团队 registry + 策略引擎（跨 agent deny 规则）
- [ ] MCP 统一管理（hooks 之外扩到 MCP，复用同一架构）

## 不做 / Non-goals

- 不替代任何 agent 本身
- 不做 agent 编排（那是 Multica 的事）
- 不自建模型代理（那是 cc-switch 的事）
