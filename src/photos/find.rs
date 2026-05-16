use crate::applescript::{escape_string, run};
use anyhow::{Context, Result, anyhow};

#[derive(Debug, Clone)]
pub struct PhotoInfo {
    pub id: String,
    pub filename: String,
    pub date: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Clone)]
pub enum Query {
    /// Filename substring match (e.g. "IMG_0595" matches IMG_0595.HEIC, IMG_0595_edit.jpg).
    Name(String),
    /// 1-based index in the library (matches the "N of M" position Photos shows).
    Index(u64),
    /// Stable photo id (UUID-like string).
    Id(String),
}

/// Find photos matching the query and return up to `limit` results.
pub fn find(query: &Query, limit: u32) -> Result<Vec<PhotoInfo>> {
    // Photos AppleScript ops on large libraries can exceed the default 2-minute
    // osascript timeout. Wrap with a generous explicit timeout.
    let script = match query {
        Query::Name(name) => {
            let lit = escape_string(name);
            format!(
                r#"
with timeout of 600 seconds
    tell application "Photos"
        set out to ""
        set hits to (media items whose filename contains "{lit}")
        set n to count of hits
        if n > {limit} then set n to {limit}
        repeat with i from 1 to n
            set p to item i of hits
            set out to out & (id of p) & tab & (filename of p) & tab & (date of p as «class isot» as string) & tab & (width of p as text) & tab & (height of p as text) & linefeed
        end repeat
        return out
    end tell
end timeout
"#,
                limit = limit
            )
        }
        Query::Index(idx) => format!(
            r#"
with timeout of 600 seconds
    tell application "Photos"
        set p to media item {idx}
        return (id of p) & tab & (filename of p) & tab & (date of p as «class isot» as string) & tab & (width of p as text) & tab & (height of p as text)
    end tell
end timeout
"#
        ),
        Query::Id(id) => {
            let lit = escape_string(id);
            format!(
                r#"
with timeout of 600 seconds
    tell application "Photos"
        set p to media item id "{lit}"
        return (id of p) & tab & (filename of p) & tab & (date of p as «class isot» as string) & tab & (width of p as text) & tab & (height of p as text)
    end tell
end timeout
"#
            )
        }
    };

    let stdout = run(&script).context("finding photos")?;
    let mut out = Vec::new();
    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 5 {
            return Err(anyhow!("unexpected photo find result: {line:?}"));
        }
        out.push(PhotoInfo {
            id: parts[0].to_string(),
            filename: parts[1].to_string(),
            date: parts[2].to_string(),
            width: parts[3].parse().unwrap_or(0),
            height: parts[4].parse().unwrap_or(0),
        });
    }
    Ok(out)
}
