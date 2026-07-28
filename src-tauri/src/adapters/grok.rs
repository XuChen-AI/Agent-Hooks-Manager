//! Grok Build adapter.
//!
//! Config: `~/.grok/hooks/*.json` (JSON). Also reads `~/.claude/settings.json`
//! for Claude compatibility - projection must avoid double-writing / overwrite.
//! Trust: global always trusted; project needs `/hooks-trust`.

use std::path::PathBuf;

use super::r#trait::{AdapterError, ConfigFormat, NativeHook, ToolAdapter, TrustModel};
use crate::model::Event;

pub struct GrokBuildAdapter;

impl ToolAdapter for GrokBuildAdapter {
    fn id(&self) -> &str {
        "grok-build"
    }
    fn display_name(&self) -> &str {
        "Grok Build"
    }
    fn is_installed(&self) -> bool {
        // TODO: `which grok`
        false
    }
    fn config_paths(&self) -> Vec<PathBuf> {
        let home = dirs::home_dir().unwrap_or_default();
        vec![home.join(".grok").join("hooks")]
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
            Event::Notification,
            Event::SessionEnd,
        ]
    }
    fn trust_model(&self) -> TrustModel {
        TrustModel::ProjectTrust
    }
    fn restart_required(&self) -> bool {
        false
    }

    fn read_hooks(&self) -> Result<Vec<NativeHook>, AdapterError> {
        // TODO: merge all ~/.grok/hooks/*.json
        Ok(Vec::new())
    }
    fn write_hooks(&self, _hooks: &[NativeHook]) -> Result<(), AdapterError> {
        // TODO: write per-hook JSON files under ~/.grok/hooks/.
        //       NOTE: do NOT also write ~/.claude/settings.json here - Grok
        //       reads it for compat; the projection layer dedupes.
        Ok(())
    }
    fn hooks_registered(&self) -> bool {
        false
    }
    fn unregister_all(&self) -> Result<(), AdapterError> {
        Ok(())
    }
}
