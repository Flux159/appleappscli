use crate::applescript::run;
use anyhow::{Context, Result};

/// List all album names in Photos.app. Output: one album name per line.
pub fn list_albums() -> Result<Vec<String>> {
    let script = r#"
with timeout of 600 seconds
    tell application "Photos"
        set out to ""
        repeat with a in albums
            set out to out & (name of a) & linefeed
        end repeat
        return out
    end tell
end timeout
"#;

    let stdout = run(script).context("listing albums")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
