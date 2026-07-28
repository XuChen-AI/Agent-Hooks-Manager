# 添加一个 Agent Adapter / Adding an Agent Adapter

agent-hooks-manager 的核心抽象是 `ToolAdapter` trait（`src-tauri/src/adapters/trait.rs`）。**加一个 agent = 加一个文件，不动核心。**

## 步骤（5 步）

1. 在 `src-tauri/src/adapters/` 新建 `<agent>.rs`，定义 `pub struct <Agent>Adapter;`
2. 为它实现 `ToolAdapter` trait（见下）
3. 在 `adapters/mod.rs` 的 `all_adapters()` 注册
4. 在 `docs/architecture.md` 的协议对照表补一行
5. 写一个集成测试：`detect → write_hooks → read_hooks` 往返一致

## ToolAdapter trait

```rust
pub trait ToolAdapter: Send + Sync {
    fn id(&self) -> &str;                     // "claude-code"
    fn display_name(&self) -> &str;           // "Claude Code"
    fn is_installed(&self) -> bool;           // 检测 CLI 在 PATH
    fn config_paths(&self) -> Vec<PathBuf>;   // ~/.claude/settings.json 等
    fn config_format(&self) -> ConfigFormat;  // Json | Toml
    fn supported_events(&self) -> &[Event];   // 该 agent 支持的事件子集
    fn trust_model(&self) -> TrustModel;      // 信任流程
    fn restart_required(&self) -> bool;       // 改 hook 后是否需重启

    fn read_hooks(&self) -> Result<Vec<NativeHook>>;
    fn write_hooks(&self, hooks: &[NativeHook]) -> Result<()>;
    fn hooks_registered(&self) -> bool;       // 是否已有 hook 注册
    fn unregister_all(&self) -> Result<()>;   // 清掉本 agent 的 hooks（残留清理用）
}
```

## 参考 / References

- weykon/agent-hooks 的 `ToolAdapter`：detect/register/unregister + 桥接脚本
- cc-switch 的 Provider preset 模式：每个 agent 一套 preset + 配置写入逻辑

## 三个 MVP adapter 的已知坑

- **Claude Code**：热切换免重启；工作区必须先信任；事件名标准。
- **Codex**：写完必须 `/hooks` 手动批准（哈希信任），UI 不能假装一键搞定；Windows hooks 支持需本机验证。
- **Grok Build**：会读 `~/.claude/settings.json`（Claude 兼容），投影层要单独处理避免误覆盖。
