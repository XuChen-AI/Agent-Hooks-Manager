//! Adapter layer - one `ToolAdapter` impl per agent.
//!
//! Adding an agent = adding a file here and registering it in `all_adapters()`.
//! See `docs/adapters.md`.

pub mod claude;
pub mod codex;
pub mod grok;

mod r#trait;
pub use r#trait::{ConfigFormat, NativeHook, ToolAdapter, TrustModel};

use std::sync::Arc;

/// All registered adapters. The GUI iterates this to detect installed agents.
pub fn all_adapters() -> Vec<Arc<dyn ToolAdapter>> {
    vec![
        Arc::new(claude::ClaudeCodeAdapter),
        Arc::new(codex::CodexAdapter),
        Arc::new(grok::GrokBuildAdapter),
        // TODO P1: Gemini, Cursor, OpenCode
    ]
}
