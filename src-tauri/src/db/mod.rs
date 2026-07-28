//! SQLite DAO layer. SSOT for all canonical hooks.
//!
//! Schema mirrors `model::Hook`. Migrations run on startup.

use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open_in_memory() -> rusqlite::Result<Self> {
        let conn = Connection::open_in_memory()?;
        Self::migrate(&conn)?;
        Ok(Self { conn })
    }

    fn migrate(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS hooks (
                id           TEXT PRIMARY KEY,
                name         TEXT NOT NULL,
                event        TEXT NOT NULL,
                matcher      TEXT,
                hook_type    TEXT NOT NULL,
                command      TEXT,
                url          TEXT,
                timeout      INTEGER,
                env          TEXT,
                enabled      INTEGER NOT NULL,
                scope        TEXT NOT NULL,
                target_agents TEXT NOT NULL,
                source       TEXT NOT NULL
            );",
        )?;
        Ok(())
    }

    // TODO: list / upsert / delete / find_by_source
}
