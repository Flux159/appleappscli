use crate::applescript::{escape_string, run};
use anyhow::{Context, Result};

/// Move a note (by id) to a different folder (by name). Folder is created if missing.
pub fn move_note(id: &str, folder: &str) -> Result<()> {
    let id_lit = escape_string(id);
    let folder_lit = escape_string(folder);

    let script = format!(
        r#"
tell application "Notes"
    set targetFolder to missing value
    repeat with f in folders
        if name of f is "{folder_lit}" then
            set targetFolder to f
            exit repeat
        end if
    end repeat
    if targetFolder is missing value then
        set targetFolder to make new folder with properties {{name:"{folder_lit}"}}
    end if
    set n to first note whose id is "{id_lit}"
    move n to targetFolder
end tell
"#
    );

    run(&script).context("moving note")?;
    Ok(())
}
