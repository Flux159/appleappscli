use crate::applescript::{escape_string, run};
use anyhow::{Context, Result, anyhow, bail};
use std::path::Path;

/// Attach a local image file to an existing note.
pub fn attach_image(id: &str, image: &Path) -> Result<()> {
    if !image.exists() {
        bail!("image file not found: {}", image.display());
    }

    let abs_path = image
        .canonicalize()
        .with_context(|| format!("resolving absolute path for {}", image.display()))?;

    let abs = abs_path
        .to_str()
        .ok_or_else(|| anyhow!("image path is not valid UTF-8: {}", abs_path.display()))?;

    let id_lit = escape_string(id);
    let path_lit = escape_string(abs);

    let script = format!(
        r#"
tell application "Notes"
    set n to first note whose id is "{id_lit}"
    make new attachment at end of attachments of n with data POSIX file "{path_lit}"
end tell
"#
    );

    run(&script).context("attaching image to note")?;
    Ok(())
}
