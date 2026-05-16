//! Read-only access to `~/Library/Messages/chat.db`.
//!
//! Requires **Full Disk Access** for the calling process (Terminal, iTerm, etc.)
//! in System Settings → Privacy & Security → Full Disk Access.

use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags, params};
use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    PathBuf::from(home).join("Library/Messages/chat.db")
}

pub fn open() -> Result<Connection> {
    let path = db_path();
    Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY).with_context(|| {
        format!(
            "opening chat.db at {} (Full Disk Access required for this terminal)",
            path.display()
        )
    })
}

/// Apple Cocoa absolute time → Unix seconds.
/// Messages stores nanoseconds since 2001-01-01 UTC; older rows store seconds.
/// Heuristic: if value > 1e12, treat as nanoseconds; else seconds.
pub fn cocoa_to_unix_seconds(cocoa: i64) -> i64 {
    const APPLE_EPOCH_OFFSET: i64 = 978_307_200; // 2001-01-01 to 1970-01-01 in seconds
    let seconds = if cocoa.abs() > 1_000_000_000_000 {
        cocoa / 1_000_000_000
    } else {
        cocoa
    };
    seconds + APPLE_EPOCH_OFFSET
}

/// Format Unix seconds as local ISO timestamp (YYYY-MM-DD HH:MM:SS).
pub fn format_local_timestamp(unix_seconds: i64) -> String {
    // Use system `date` to format — avoids pulling in a date crate just for this.
    let out = std::process::Command::new("date")
        .args(["-r", &unix_seconds.to_string(), "+%Y-%m-%d %H:%M:%S"])
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => format!("@{unix_seconds}"),
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // guid kept available for future operations (e.g. precise chat lookup)
pub struct ChatRow {
    pub guid: String,
    pub chat_identifier: String,
    pub display_name: String,
    pub last_message_cocoa: i64,
    pub last_message_text: String,
}

/// List chats sorted by most-recent message first (matches Messages.app UI order).
pub fn list_chats(conn: &Connection, limit: u32) -> Result<Vec<ChatRow>> {
    let sql = r#"
SELECT
  c.guid,
  c.chat_identifier,
  COALESCE(c.display_name, '') AS display_name,
  MAX(m.date) AS last_date,
  COALESCE((
    SELECT m2.text FROM message m2
    JOIN chat_message_join cmj2 ON cmj2.message_id = m2.ROWID
    WHERE cmj2.chat_id = c.ROWID
    ORDER BY m2.date DESC LIMIT 1
  ), '') AS last_text
FROM chat c
JOIN chat_message_join cmj ON cmj.chat_id = c.ROWID
JOIN message m ON m.ROWID = cmj.message_id
GROUP BY c.ROWID
ORDER BY last_date DESC
LIMIT ?
"#;

    let mut stmt = conn.prepare(sql).context("preparing list_chats query")?;
    let rows = stmt
        .query_map(params![limit], |row| {
            Ok(ChatRow {
                guid: row.get(0)?,
                chat_identifier: row.get(1)?,
                display_name: row.get(2)?,
                last_message_cocoa: row.get::<_, Option<i64>>(3)?.unwrap_or(0),
                last_message_text: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
            })
        })
        .context("running list_chats query")?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // rowid kept available for future operations (e.g. reply-to threading)
pub struct MessageRow {
    pub rowid: i64,
    pub date_cocoa: i64,
    pub is_from_me: bool,
    pub sender: String,
    pub text: String,
}

/// Read messages from a chat. Chat is identified by either guid or chat_identifier
/// (phone number / email / group chat name). Returns most-recent N messages.
pub fn read_messages(conn: &Connection, chat_key: &str, limit: u32) -> Result<Vec<MessageRow>> {
    let sql = r#"
SELECT
  m.ROWID,
  m.date,
  m.is_from_me,
  COALESCE(h.id, '') AS sender,
  COALESCE(m.text, '') AS text
FROM message m
JOIN chat_message_join cmj ON cmj.message_id = m.ROWID
JOIN chat c ON c.ROWID = cmj.chat_id
LEFT JOIN handle h ON h.ROWID = m.handle_id
WHERE c.guid = ?1 OR c.chat_identifier = ?1
ORDER BY m.date DESC
LIMIT ?2
"#;

    let mut stmt = conn.prepare(sql).context("preparing read_messages query")?;
    let rows = stmt
        .query_map(params![chat_key, limit], |row| {
            Ok(MessageRow {
                rowid: row.get(0)?,
                date_cocoa: row.get(1)?,
                is_from_me: row.get::<_, i64>(2)? != 0,
                sender: row.get(3)?,
                text: row.get(4)?,
            })
        })
        .context("running read_messages query")?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    out.reverse(); // return oldest-first for display
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cocoa_seconds_form() {
        // 2026-05-16 00:00:00 UTC ≈ Cocoa seconds 769305600
        let unix = cocoa_to_unix_seconds(769305600);
        assert_eq!(unix, 769305600 + 978_307_200);
    }

    #[test]
    fn cocoa_nanoseconds_form() {
        // Same date in nanoseconds
        let unix = cocoa_to_unix_seconds(769_305_600_000_000_000);
        assert_eq!(unix, 769_305_600 + 978_307_200);
    }
}
