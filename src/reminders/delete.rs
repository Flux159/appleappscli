use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Delete a reminder by id.
pub fn delete_reminder(id: &str) -> Result<()> {
    let id_lit = escape_string(id);
    let script = format!(
        r#"
tell application "Reminders"
    set r to first reminder whose id is "{id_lit}"
    delete r
end tell
"#
    );

    run(&script).context("deleting reminder")?;
    Ok(())
}
