use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Mark a reminder as completed by id.
pub fn complete_reminder(id: &str) -> Result<()> {
    let id_lit = escape_string(id);
    let script = format!(
        r#"
tell application "Reminders"
    set r to first reminder whose id is "{id_lit}"
    set completed of r to true
end tell
"#
    );

    run(&script).context("completing reminder")?;
    Ok(())
}
