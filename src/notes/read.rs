use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Fetch the HTML body of a note by id.
pub fn read_note(id: &str) -> Result<String> {
    let id_lit = escape_string(id);
    let script = format!(
        r#"
tell application "Notes"
    set n to first note whose id is "{id_lit}"
    return body of n
end tell
"#
    );

    run(&script).context("reading note")
}
