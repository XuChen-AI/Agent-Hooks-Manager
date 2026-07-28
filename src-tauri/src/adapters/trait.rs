//! The core abstraction. Every agent implements this so the projection layer
//! can treat them uniformly. Inspired by weykon/agent-hooks' `ToolAdapter`.

use std::path::PathBuf;

use crate::model::{Event, HookType, Scope};

/// Native config format an agent stores its hooks in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Json,
    Toml,
}

/// How an agent gates hook execution (must be surfaced in the GUI).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustModel {
    /// Claude Code: workspace must be trusted.
    WorkspaceTrust,
    /// Codex: hash-based, user must run `/hooks` to approve.
    HashApproval,
    /// Grok Build: global always trusted, project needs `/hooks-trust`.
    ProjectTrust,
    /// No trust gate (e.g. some agents).
    None,
}

/// A hook as it lives in an agent's native config file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NativeHook {
    pub event: Event,
    pub matcher: Option<String>,
    pub hook_type: HookType,
    pub command: Option<String>,
    pub url: Option<String>,
    pub timeout: Option<u32>,
    pub env: std::collections::HashMap<String, String>,
    pub scope: Scope,
}

/// Implement this for each agent. One file per agent, no core changes needed.
pub trait ToolAdapter: Send + Sync {
    /// Stable id, e.g. `"claude-code"`.
    fn id(&self) -> &str;
    /// Human-readable name, e.g. `"Claude Code"`.
    fn display_name(&self) -> &str;
    /// Is the agent's CLI installed / detectable on this machine?
    fn is_installed(&self) -> bool;
    /// Config file paths this agent reads hooks from (user + project scopes).
    fn config_paths(&self) -> Vec<PathBuf>;
    /// Format of those config files.
    fn config_format(&self) -> ConfigFormat;
    /// Subset of events this agent supports.
    fn supported_events(&self) -> &'static [Event];
    /// How this agent gates hook execution.
    fn trust_model(&self) -> TrustModel;
    /// Whether the agent must be restarted after hooks change.
    fn restart_required(&self) -> bool;

    /// Read all hooks currently in this agent's native config.
    fn read_hooks(&self) -> Result<Vec<NativeHook>, AdapterError>;
    /// Write the given hooks into the agent's native config (replaces).
    fn write_hooks(&self, hooks: &[NativeHook]) -> Result<(), AdapterError>;
    /// Whether any hooks are currently registered.
    fn hooks_registered(&self) -> bool;
    /// Remove all hooks written by agent-hooks-manager (residue cleanup).
    fn unregister_all(&self) -> Result<(), AdapterError>;
}

#[derive(Debug, thiserror::Error)]
pub enum AdapterError {
    #[error("config file not found: {0}")]
    NotFound(PathBuf),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("not installed")]
    NotInstalled,
}
