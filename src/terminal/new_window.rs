use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Open a new Terminal window. If `command` is provided, run it in the new window.
/// Returns the window's id (numeric).
pub fn new_window(command: Option<&str>) -> Result<String> {
    // `do script ""` creates a new Terminal window without running anything.
    // `do script "cmd"` creates a new window running `cmd`.
    let cmd_lit = escape_string(command.unwrap_or(""));

    let script = format!(
        r#"
tell application "Terminal"
    activate
    set w to do script "{cmd_lit}"
    return (id of (window 1) as text)
end tell
"#
    );

    run(&script).context("opening new Terminal window")
}
