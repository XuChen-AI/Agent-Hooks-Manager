//! Sync layer - bidirectional backfill + atomic writes.
//!
//! On deploy: write live config files (temp file + rename = atomic, corruption-safe).
//! On read: backfill from live files so manual edits outside agent-hooks-manager are detected.
//! Mirrors cc-switch's sync model.

use std::path::Path;

/// Write `bytes` to `path` atomically: write to a temp sibling, then rename.
/// Prevents config corruption if the process is killed mid-write.
pub fn atomic_write(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    // TODO: temp file in same dir + persist/rename. On Windows use PersistableFileHandle.
    let _ = (path, bytes);
    Ok(())
}

/// Reconcile in-memory state with what's actually on disk right now.
/// Returns hooks found on disk that agent-hooks-manager didn't know about (user edits /
/// third-party installs) - feeds residue detection.
pub fn backfill(/* db, adapter */) -> Vec<String> {
    // TODO: diff DB vs live config, return unknown hook fingerprints.
    Vec::new()
}
