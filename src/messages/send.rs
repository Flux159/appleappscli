use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Send a message via Messages.app.
/// Recipient may be a phone number (E.164 preferred), email, or chat handle.
pub fn send_message(recipient: &str, text: &str) -> Result<()> {
    let recip_lit = escape_string(recipient);
    let text_lit = escape_string(text);

    // Prefer iMessage service; fall back to first available service if iMessage
    // doesn't have a buddy for this recipient.
    let script = format!(
        r#"
tell application "Messages"
    set targetService to missing value
    repeat with s in services
        if service type of s is iMessage then
            set targetService to s
            exit repeat
        end if
    end repeat
    if targetService is missing value then
        set targetService to first service
    end if
    set targetBuddy to buddy "{recip_lit}" of targetService
    send "{text_lit}" to targetBuddy
end tell
"#
    );

    run(&script).context("sending message")?;
    Ok(())
}
