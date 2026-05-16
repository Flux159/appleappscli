use crate::applescript::run;
use anyhow::{Context, Result};

/// List all (account, mailbox) pairs. Output per line: `<account>\t<mailbox>`.
pub fn list_mailboxes() -> Result<Vec<String>> {
    let script = r#"
tell application "Mail"
    set out to ""
    repeat with a in accounts
        repeat with m in mailboxes of a
            set out to out & (name of a) & tab & (name of m) & linefeed
        end repeat
    end repeat
    return out
end tell
"#;

    let stdout = run(script).context("listing mailboxes")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
