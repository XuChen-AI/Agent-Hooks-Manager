//! Projection layer - canonical `Hook` -> each agent's native config.
//!
//! Mirrors Hookbridge's universal->native compilation with a loss-report:
//! any event/matcher an agent doesn't support is flagged, never silently dropped.

use crate::adapters::{NativeHook, ToolAdapter};
use crate::model::Hook;

/// Something that couldn't be projected cleanly to an agent.
#[derive(Debug, Clone)]
pub struct ProjectionGap {
    pub agent_id: String,
    pub hook_name: String,
    pub reason: String,
}

/// Project a canonical hook into the native shape for the given adapter.
/// Returns `None` (and a gap) if the agent doesn't support the hook's event.
pub fn project(hook: &Hook, adapter: &dyn ToolAdapter) -> Result<Option<NativeHook>, ProjectionGap> {
    if !adapter.supported_events().contains(&hook.event) {
        return Err(ProjectionGap {
            agent_id: adapter.id().to_string(),
            hook_name: hook.name.clone(),
            reason: format!("agent does not support event {:?}", hook.event),
        });
    }
    // TODO: translate matcher syntax per agent (Grok uses regex, Claude uses
    //       permissions-style, Codex differs). Produce loss-report entries.
    Ok(Some(NativeHook {
        event: hook.event,
        matcher: hook.matcher.clone(),
        hook_type: hook.hook_type,
        command: hook.command.clone(),
        url: hook.url.clone(),
        timeout: hook.timeout,
        env: hook.env.clone(),
        scope: hook.scope,
    }))
}

/// Project + write a set of hooks to one adapter. Returns all gaps encountered.
pub fn deploy(hooks: &[Hook], adapter: &dyn ToolAdapter) -> Vec<ProjectionGap> {
    let mut gaps = Vec::new();
    let mut native = Vec::new();
    for h in hooks {
        if !h.target_agents.contains(&adapter.id().to_string()) {
            continue;
        }
        match project(h, adapter) {
            Ok(Some(n)) => native.push(n),
            Ok(None) => {}
            Err(g) => gaps.push(g),
        }
    }
    // adapter.write_hooks(&native) // TODO
    let _ = &mut native;
    gaps
}
