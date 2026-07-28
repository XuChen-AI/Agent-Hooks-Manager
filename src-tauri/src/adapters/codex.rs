//! Codex CLI adapter.
//!
//! Config: `~/.codex/hooks.json` (JSON). Requires restart.
//! Trust: hash-based - user MUST run `/hooks` in Codex to approve after writes.
//! Caveat: Windows hooks support was temporarily disabled in some versions -
//! must verify on the host before promising Codex support on Windows.

use std::path::PathBuf;

use super::r#trait::{AdapterError, ConfigFormat, NativeHook, ToolAdapter, TrustModel};
use crate::model::Event;

pub struct CodexAdapter;

impl ToolAdapter for CodexAdapter {
    fn id(&self) -> &str {
        "codex"
    }
    fn display_name(&self) -> &str {
        "Codex"
    }
    fn is_installed(&self) -> bool {
        // TODO: `which codex`
        false
    }
    fn config_paths(&self) -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        vec![home.join(".codex").join("hooks.json")]
    }
    fn config_format(&self) -> ConfigFormat {
        ConfigFormat::Json
    }
    fn supported_events(&self) -> &'static [Event] {
        &[
            Event::SessionStart,
            Event::PreToolUse,
            Event::PostToolUse,
            Event::UserPromptSubmit,
            Event::Stop,
        ]
    }
    fn trust_model(&self) -> TrustModel {
        TrustModel::HashApproval
    }
    fn restart_required(&self) -> bool {
        true
    }

    fn read_hooks(&self) -> Result<Vec<NativeHook>, AdapterError> {
        // TODO
        Ok(Vec::new())
    }
    fn write_hooks(&self, _hooks: &[NativeHook]) -> Result<(), AdapterError> {
        // TODO: write, then the GUI must prompt user to run `/hooks` in Codex.
        Ok(())
    }
    fn hooks_registered(&self) -> bool {
        false
    }
    fn unregister_all(&self) -> Result<(), AdapterError> {
        Ok(())
    }
}
