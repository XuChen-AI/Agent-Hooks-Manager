//! Trust & safety layer.
//!
//! Each agent has a non-bypassable trust gate after hooks are written. The GUI
//! must surface this - it cannot pretend "one click" works when it doesn't.

use crate::adapters::TrustModel;

/// What the user must do (manually) after agent-hooks-manager writes hooks to this agent.
pub fn post_deploy_instruction(model: TrustModel) -> &'static str {
    match model {
        TrustModel::WorkspaceTrust => "Open the project in the agent and accept the workspace trust prompt.",
        TrustModel::HashApproval => "Run `/hooks` inside Codex and approve the new/changed hooks (hash trust).",
        TrustModel::ProjectTrust => "Run `/hooks-trust` for this project (or open the /hooks modal).",
        TrustModel::None => "No action needed - hooks take effect on next session.",
    }
}

/// Detect third-party residue: hooks on disk whose `source` is ThirdParty and
/// whose owning tool is no longer installed. Prime candidate for cleanup.
pub fn detect_residue(/* adapters, db */) -> Vec<String> {
    // TODO: for each adapter, read native hooks; flag any whose command path
    // points into a directory of an uninstalled tool (e.g. ~/.orca/...).
    Vec::new()
}
