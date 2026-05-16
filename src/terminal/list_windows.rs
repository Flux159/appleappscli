use crate::applescript::run;
use anyhow::{Context, Result};

/// List Terminal windows. Output per line: `<window_id>\t<tab_count>\t<custom_title>\t<tty>`
pub fn list_windows() -> Result<Vec<String>> {
    let script = r#"
tell application "Terminal"
    set out to ""
    repeat with w in windows
        set wid to id of w as text
        set tabCount to (count of tabs of w) as text
        set ti to ""
        try
            set ti to custom title of selected tab of w
        end try
        if ti is missing value or ti is "" then set ti to "-"
        set tt to ""
        try
            set tt to tty of selected tab of w
        end try
        if tt is missing value then set tt to ""
        set out to out & wid & tab & tabCount & tab & ti & tab & tt & linefeed
    end repeat
    return out
end tell
"#;

    let stdout = run(script).context("listing Terminal windows")?;
    Ok(stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.to_string())
        .collect())
}
