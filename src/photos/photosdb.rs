//! Read-only access to the Photos library SQLite database at
//! `~/Pictures/Photos Library.photoslibrary/database/Photos.sqlite`.
//!
//! Requires **Full Disk Access** for the calling process (Terminal, iTerm, etc.)
//! in System Settings → Privacy & Security → Full Disk Access.
//!
//! The DB is opened in URI mode with `mode=ro&immutable=1` so we can read safely
//! while Photos.app has the database open — we don't touch the WAL.

use anyhow::{Context, Result};
use rusqlite::{Connection, OpenFlags, params};
use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/".to_string());
    PathBuf::from(home).join("Pictures/Photos Library.photoslibrary/database/Photos.sqlite")
}

pub fn open() -> Result<Connection> {
    let path = db_path();
    // URI mode lets us pass `immutable=1`, which tells SQLite to skip locking
    // and ignore the WAL. Safe because we're strictly read-only.
    let uri = format!("file:{}?mode=ro&immutable=1", path.display());
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI;
    Connection::open_with_flags(&uri, flags).with_context(|| {
        format!(
            "opening Photos.sqlite at {} (Full Disk Access required for this terminal)",
            path.display()
        )
    })
}

/// Cocoa absolute time (seconds since 2001-01-01 UTC) → "YYYY-MM-DD HH:MM:SS" local.
pub fn format_cocoa_local(cocoa_seconds: f64) -> String {
    const APPLE_EPOCH_OFFSET: i64 = 978_307_200;
    let unix = cocoa_seconds as i64 + APPLE_EPOCH_OFFSET;
    let out = std::process::Command::new("date")
        .args(["-r", &unix.to_string(), "+%Y-%m-%d %H:%M:%S"])
        .output();
    match out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => format!("@{unix}"),
    }
}

#[derive(Debug, Clone)]
pub struct PhotoRow {
    pub uuid: String,
    pub original_filename: String,
    pub date_local: String,
    pub width: i64,
    pub height: i64,
}

const SELECT_COLS: &str = "a.ZUUID, COALESCE(aa.ZORIGINALFILENAME, a.ZFILENAME), a.ZDATECREATED, COALESCE(a.ZWIDTH, 0), COALESCE(a.ZHEIGHT, 0)";

fn map_row(row: &rusqlite::Row) -> rusqlite::Result<PhotoRow> {
    let date_cocoa: Option<f64> = row.get(2)?;
    Ok(PhotoRow {
        uuid: row.get(0)?,
        original_filename: row.get::<_, Option<String>>(1)?.unwrap_or_default(),
        date_local: date_cocoa.map(format_cocoa_local).unwrap_or_default(),
        width: row.get(3)?,
        height: row.get(4)?,
    })
}

/// Find by filename substring match (e.g. "IMG_0595" matches IMG_0595.HEIC).
/// Matches against ZADDITIONALASSETATTRIBUTES.ZORIGINALFILENAME (the user-visible
/// filename), not ZASSET.ZFILENAME (which is a UUID-derived internal name).
pub fn find_by_name(conn: &Connection, name: &str, limit: u32) -> Result<Vec<PhotoRow>> {
    let pattern = format!("%{name}%");
    let sql = format!(
        r#"
SELECT {SELECT_COLS}
FROM ZASSET a
JOIN ZADDITIONALASSETATTRIBUTES aa ON aa.ZASSET = a.Z_PK
WHERE a.ZTRASHEDSTATE = 0 AND aa.ZORIGINALFILENAME LIKE ?1
ORDER BY a.ZDATECREATED DESC
LIMIT ?2
"#
    );
    let mut stmt = conn.prepare(&sql).context("preparing find_by_name")?;
    let rows = stmt
        .query_map(params![pattern, limit], map_row)
        .context("running find_by_name")?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}

/// Find by 1-based library index. Matches the chronological position in Photos
/// (sorted by date ascending — same order Photos shows in "All Photos").
pub fn find_by_index(conn: &Connection, index: u64) -> Result<Option<PhotoRow>> {
    if index == 0 {
        return Ok(None);
    }
    let sql = format!(
        r#"
SELECT {SELECT_COLS}
FROM ZASSET a
LEFT JOIN ZADDITIONALASSETATTRIBUTES aa ON aa.ZASSET = a.Z_PK
WHERE a.ZTRASHEDSTATE = 0
ORDER BY a.ZDATECREATED ASC, a.Z_PK ASC
LIMIT 1 OFFSET ?1
"#
    );
    let offset = index - 1;
    let mut stmt = conn.prepare(&sql).context("preparing find_by_index")?;
    let mut rows = stmt
        .query_map(params![offset as i64], map_row)
        .context("running find_by_index")?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

/// Find by stable UUID.
pub fn find_by_uuid(conn: &Connection, uuid: &str) -> Result<Option<PhotoRow>> {
    let sql = format!(
        r#"
SELECT {SELECT_COLS}
FROM ZASSET a
LEFT JOIN ZADDITIONALASSETATTRIBUTES aa ON aa.ZASSET = a.Z_PK
WHERE a.ZUUID = ?1
LIMIT 1
"#
    );
    let mut stmt = conn.prepare(&sql).context("preparing find_by_uuid")?;
    let mut rows = stmt
        .query_map(params![uuid], map_row)
        .context("running find_by_uuid")?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

/// List user-created album titles (ZKIND=2). System collections (memories,
/// imports, projects, etc.) are excluded — they aren't what people mean by
/// "my albums".
pub fn list_user_albums(conn: &Connection) -> Result<Vec<String>> {
    let sql = r#"
SELECT ZTITLE
FROM ZGENERICALBUM
WHERE ZKIND = 2 AND ZTRASHEDSTATE = 0 AND ZTITLE IS NOT NULL
ORDER BY ZTITLE COLLATE NOCASE ASC
"#;
    let mut stmt = conn.prepare(sql).context("preparing list_user_albums")?;
    let rows = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .context("running list_user_albums")?;
    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
