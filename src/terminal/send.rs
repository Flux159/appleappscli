use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Send a command to a specific Terminal window (and optionally a tab within it).
/// If `window_id` is None, sends to the front window.
pub fn send_command(window_id: Option<&str>, tab_index: Option<u32>, command: &str) -> Result<()> {
    let cmd_lit = escape_string(command);

    let target = match (window_id, tab_index) {
        (Some(wid), Some(t)) => {
            let wid_lit = escape_string(wid);
            format!("tab {t} of (first window whose id is {wid_lit})")
        }
        (Some(wid), None) => {
            let wid_lit = escape_string(wid);
            format!("selected tab of (first window whose id is {wid_lit})")
        }
        (None, Some(t)) => format!("tab {t} of front window"),
        (None, None) => "selected tab of front window".to_string(),
    };

    let script = format!(
        r#"
tell application "Terminal"
    do script "{cmd_lit}" in {target}
end tell
"#
    );

    run(&script).context("sending command to Terminal")?;
    Ok(())
}
