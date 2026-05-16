use crate::applescript::run;
use anyhow::{Context, Result};

/// List all calendar names accessible to Calendar.app.
pub fn list_calendars() -> Result<Vec<String>> {
    let script = r#"
tell application "Calendar"
    set out to ""
    repeat with c in calendars
        set out to out & (name of c) & linefeed
    end repeat
    return out
end tell
"#;

    let stdout = run(script).context("listing calendars")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
