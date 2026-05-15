use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// List notes optionally filtered by folder name.
/// Output format per line: `<note id>\t<note name>`
pub fn list_notes(folder: Option<&str>) -> Result<Vec<String>> {
    let script = match folder {
        Some(name) => {
            let folder_lit = escape_string(name);
            format!(
                r#"
tell application "Notes"
    set out to ""
    repeat with f in folders
        if name of f is "{folder_lit}" then
            repeat with n in notes of f
                set out to out & (id of n) & tab & (name of n) & linefeed
            end repeat
            exit repeat
        end if
    end repeat
    return out
end tell
"#
            )
        }
        None => r#"
tell application "Notes"
    set out to ""
    repeat with n in notes
        set out to out & (id of n) & tab & (name of n) & linefeed
    end repeat
    return out
end tell
"#
        .to_string(),
    };

    let stdout = run(&script).context("listing notes")?;
    let lines = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect();
    Ok(lines)
}
