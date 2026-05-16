use crate::applescript::run;
use anyhow::{Context, Result};

/// List recent inbox messages across all accounts. Output per line:
/// `<account>\t<message_id>\t<date>\t<sender>\t<subject>` (most recent per-account first).
pub fn list_recent(limit: u32) -> Result<Vec<String>> {
    // Iterate each account's inbox mailbox instead of using the unified `inbox`
    // keyword, which is unreliable across macOS versions and returns -1741 in
    // some configurations.
    let script = format!(
        r#"
with timeout of 300 seconds
    tell application "Mail"
        set out to ""
        repeat with a in accounts
            set acctName to name of a
            try
                set inboxMb to (mailbox "INBOX" of a)
            on error
                try
                    set inboxMb to (first mailbox of a whose name is "INBOX")
                on error
                    set inboxMb to missing value
                end try
            end try
            if inboxMb is not missing value then
                set msgs to messages of inboxMb
                set n to count of msgs
                if n > {limit} then set n to {limit}
                repeat with i from 1 to n
                    try
                        set m to item i of msgs
                        set mid to (id of m) as text
                        set mdate to ""
                        try
                            set mdate to (date received of m) as string
                        end try
                        set msender to ""
                        try
                            set msender to sender of m
                        end try
                        set msubject to ""
                        try
                            set msubject to subject of m
                        end try
                        set out to out & acctName & tab & mid & tab & mdate & tab & msender & tab & msubject & linefeed
                    end try
                end repeat
            end if
        end repeat
        return out
    end tell
end timeout
"#,
        limit = limit
    );

    let stdout = run(&script).context("listing recent mail")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
