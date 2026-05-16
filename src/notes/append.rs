use crate::applescript::{escape_string, run};
use crate::markdown::to_html;
use anyhow::{Context, Result, anyhow};
use std::io::Read;
use std::path::PathBuf;

pub struct AppendOptions {
    pub id: String,
    pub markdown_file: Option<PathBuf>,
    pub html_body: Option<String>,
    pub stdin: bool,
}

/// Append HTML content to an existing note's body.
pub fn append_note(opts: &AppendOptions) -> Result<()> {
    let html = resolve_body(opts)?;

    let id_lit = escape_string(&opts.id);
    let body_lit = escape_string(&html);

    let script = format!(
        r#"
tell application "Notes"
    set n to first note whose id is "{id_lit}"
    set body of n to ((body of n) & "{body_lit}")
end tell
"#
    );

    run(&script).context("appending to note")?;
    Ok(())
}

fn resolve_body(opts: &AppendOptions) -> Result<String> {
    if let Some(html) = &opts.html_body {
        return Ok(html.clone());
    }
    if opts.stdin {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("reading stdin")?;
        return Ok(to_html(&buf));
    }
    if let Some(path) = &opts.markdown_file {
        let md = std::fs::read_to_string(path)
            .with_context(|| format!("reading markdown file {}", path.display()))?;
        return Ok(to_html(&md));
    }
    Err(anyhow!(
        "must supply one of --markdown-file, --html-body, or --stdin"
    ))
}
