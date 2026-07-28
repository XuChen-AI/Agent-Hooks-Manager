//! Claude Code adapter.
//!
//! Config: `~/.claude/settings.json` (JSON). Hot-swap, no restart.
//! Trust: workspace trust. Standard event names.

use std::path::PathBuf;

use super::r#trait::{AdapterError, ConfigFormat, NativeHook, ToolAdapter, TrustModel};
use crate::model::Event;

pub struct ClaudeCodeAdapter;

impl ToolAdapter for ClaudeCodeAdapter {
    fn id(&self) -> &str {
        "claude-code"
    }
    fn display_name(&self) -> &str {
        "Claude Code"
    }
    fn is_installed(&self) -> bool {
        // TODO: `which claude` / check PATH
        false
    }
    fn config_paths(&self) -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        vec![home.join(".claude").join("settings.json")]
    }
    fn config_format(&self) -> ConfigFormat {
        ConfigFormat::Json
    }
    fn supported_events(&self) -> &'static [Event] {
        &[
            Event::SessionStart,
            Event::UserPromptSubmit,
            Event::PreToolUse,
            Event::PostToolUse,
            Event::Stop,
            Event::SubagentStop,
            Event::Notification,
            Event::PreCompact,
            Event::SessionEnd,
        ]
    }
    fn trust_model(&self) -> TrustModel {
        TrustModel::WorkspaceTrust
    }
    fn restart_required(&self) -> bool {
        false
    }

    fn read_hooks(&self) -> Result<Vec<NativeHook>, AdapterError> {
        // TODO: parse settings.json -> hooks.* -> Vec<NativeHook>
        Ok(Vec::new())
    }
    fn write_hooks(&self, _hooks: &[NativeHook]) -> Result<(), AdapterError> {
        // TODO: structured JSON merge into settings.json (preserve other keys),
        //       atomic write (temp + rename).
        Ok(())
    }
    fn hooks_registered(&self) -> bool {
        // TODO
        false
    }
    fn unregister_all(&self) -> Result<(), AdapterError> {
        // TODO: remove only agent-hooks-manager-tagged hooks, keep user hooks.
        Ok(())
    }
}
