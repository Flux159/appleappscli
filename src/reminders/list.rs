use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// List reminders. Output per line: `<reminder id>\t<name>\t<completed: true|false>`
pub fn list_reminders(list: Option<&str>, include_completed: bool) -> Result<Vec<String>> {
    let target_clause = match list {
        Some(name) => format!(
            "set targetList to first list whose name is \"{}\"",
            escape_string(name)
        ),
        None => "set targetList to default list".to_string(),
    };

    let filter = if include_completed {
        "reminders of targetList".to_string()
    } else {
        "(reminders of targetList whose completed is false)".to_string()
    };

    let script = format!(
        r#"
tell application "Reminders"
    {target_clause}
    set out to ""
    repeat with r in {filter}
        set out to out & (id of r) & tab & (name of r) & tab & (completed of r as text) & linefeed
    end repeat
    return out
end tell
"#
    );

    let stdout = run(&script).context("listing reminders")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
