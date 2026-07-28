//! Canonical hook model - the single source of truth stored in SQLite.
//!
//! A canonical `Hook` is agent-agnostic. The projection layer translates it
//! into each target agent's native format.

use std::collections::HashMap;

use uuid::Uuid;

/// Superset of all hook events across supported agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Event {
    SessionStart,
    UserPromptSubmit,
    PreToolUse,
    PostToolUse,
    Stop,
    SubagentStop,
    Notification,
    PreCompact,
    SessionEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum HookType {
    Command,
    Http,
    Prompt,
    Agent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Scope {
    Global,
    Project,
}

/// Where a hook came from - drives residue detection.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Source {
    /// Authored by the user via agent-hooks-manager.
    User,
    /// Left behind by a third-party tool (e.g. orca). Flagged for cleanup.
    ThirdParty { tool_name: String },
    /// Installed from the preset marketplace.
    Preset { preset_id: String },
}

/// The canonical hook entity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Hook {
    pub id: Uuid,
    pub name: String,
    pub event: Event,
    pub matcher: Option<String>,
    pub hook_type: HookType,
    pub command: Option<String>,
    pub url: Option<String>,
    pub timeout: Option<u32>,
    pub env: HashMap<String, String>,
    pub enabled: bool,
    pub scope: Scope,
    /// Which agents this hook deploys to.
    pub target_agents: Vec<String>,
    pub source: Source,
}
