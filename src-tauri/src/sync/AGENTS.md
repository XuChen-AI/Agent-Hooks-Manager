<!-- deepinit:generated start -->
<!-- Parent: ../AGENTS.md -->

# sync

## Purpose
Sync layer - atomic config writes and bidirectional backfill. On deploy it writes live config files corruption-safely; on read it reconciles in-memory state with disk so manual or third-party edits are detected.

## Key Files
| File | Description |
|------|-------------|
| `mod.rs` | `atomic_write()` (temp file + rename) and `backfill()` (diff DB vs live config, return unknown hook fingerprints) - both TODO stubs |

## Subdirectories
No subdirectories.

## For AI Agents
`atomic_write()` must use a temp sibling file + rename (with a Windows-specific handle path) to avoid corrupting agent configs if the process is killed mid-write. `backfill()` feeds residue detection by returning hooks found on disk that the DB doesn't know about. Both functions are currently stubs. Modeled on cc-switch's sync approach.

<!-- deepinit:generated end -->

## Manual Notes
