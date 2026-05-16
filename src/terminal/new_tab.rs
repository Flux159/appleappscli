use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Open a new tab in Terminal's front window. If no Terminal window exists,
/// opens a new window instead. Returns the window id (the new tab is the
/// selected tab of that window).
pub fn new_tab(command: Option<&str>) -> Result<String> {
    let cmd_lit = escape_string(command.unwrap_or(""));

    // Terminal.app doesn't expose tab-creation directly via AppleScript.
    // The reliable pattern is to send Cmd+T to System Events when Terminal is
    // frontmost, then `do script ... in selected tab of front window` (which now
    // targets the new tab).
    let script = format!(
        r#"
tell application "Terminal"
    activate
    if (count windows) is 0 then
        do script "{cmd_lit}"
        return (id of front window) as text
    end if
end tell
tell application "System Events" to keystroke "t" using {{command down}}
delay 0.25
tell application "Terminal"
    do script "{cmd_lit}" in selected tab of front window
    return (id of front window) as text
end tell
"#
    );

    run(&script).context("opening new Terminal tab")
}
