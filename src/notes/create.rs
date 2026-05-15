use crate::applescript::{escape_string, run};
use crate::markdown::to_html;
use anyhow::{Context, Result, anyhow};
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct CreateOptions {
    pub markdown_file: Option<PathBuf>,
    pub html_body: Option<String>,
    pub stdin: bool,
    pub folder: Option<String>,
    pub title: Option<String>,
}

/// Create a new Apple Note. Returns the note's id (e.g. `x-coredata://...`).
pub fn create_note(opts: &CreateOptions) -> Result<String> {
    let (html_body, derived_title) = resolve_body(opts)?;
    let title = opts.title.clone().or(derived_title);

    // Apple Notes derives the displayed title from the FIRST line of the body.
    // To ensure a stable title (matching the file or override), we prepend
    // an H1 with the title if one isn't already present.
    let final_body = ensure_title_in_body(&html_body, title.as_deref());

    let script = build_create_script(&final_body, opts.folder.as_deref())?;
    let stdout = run(&script).context("creating note")?;

    // AppleScript's `id` return looks like: note id "x-coredata://..."
    // Strip wrapper if present.
    Ok(stdout)
}

fn resolve_body(opts: &CreateOptions) -> Result<(String, Option<String>)> {
    if let Some(html) = &opts.html_body {
        return Ok((html.clone(), None));
    }
    if opts.stdin {
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("reading stdin")?;
        let html = to_html(&buf);
        let derived = extract_first_heading(&buf);
        return Ok((html, derived));
    }
    if let Some(path) = &opts.markdown_file {
        let md = std::fs::read_to_string(path)
            .with_context(|| format!("reading markdown file {}", path.display()))?;
        let html = to_html(&md);
        let derived = extract_first_heading(&md).or_else(|| stem_title(path));
        return Ok((html, derived));
    }
    Err(anyhow!(
        "must supply one of --markdown-file, --html-body, or --stdin"
    ))
}

fn extract_first_heading(md: &str) -> Option<String> {
    for line in md.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            return Some(rest.trim().to_string());
        }
        if let Some(rest) = trimmed.strip_prefix("## ") {
            return Some(rest.trim().to_string());
        }
    }
    None
}

fn stem_title(path: &Path) -> Option<String> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .map(|s| s.to_string())
}

fn ensure_title_in_body(html: &str, title: Option<&str>) -> String {
    let Some(title) = title else {
        return html.to_string();
    };
    // If body already starts with an <h1>, leave alone.
    let trimmed = html.trim_start();
    if trimmed.starts_with("<h1>") || trimmed.starts_with("<h1 ") {
        return html.to_string();
    }
    format!("<h1>{}</h1>{}", html_escape(title), html)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn build_create_script(html_body: &str, folder: Option<&str>) -> Result<String> {
    let body_lit = escape_string(html_body);

    let script = match folder {
        Some(name) => {
            let folder_lit = escape_string(name);
            format!(
                r#"
tell application "Notes"
    set folderName to "{folder_lit}"
    set targetFolder to missing value
    repeat with f in folders
        if name of f is folderName then
            set targetFolder to f
            exit repeat
        end if
    end repeat
    if targetFolder is missing value then
        set targetFolder to make new folder with properties {{name:folderName}}
    end if
    tell targetFolder
        set newNote to make new note with properties {{body:"{body_lit}"}}
    end tell
    return id of newNote
end tell
"#
            )
        }
        None => format!(
            r#"
tell application "Notes"
    set newNote to make new note with properties {{body:"{body_lit}"}}
    return id of newNote
end tell
"#
        ),
    };

    Ok(script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_heading_extracted() {
        let md = "Random preamble\n\n# Real Title\n\nbody";
        assert_eq!(extract_first_heading(md).as_deref(), Some("Real Title"));
    }

    #[test]
    fn second_heading_used_if_no_h1() {
        let md = "## Just H2\n\nbody";
        assert_eq!(extract_first_heading(md).as_deref(), Some("Just H2"));
    }

    #[test]
    fn title_prepended_when_missing() {
        let body = "<p>hello</p>";
        let out = ensure_title_in_body(body, Some("My Title"));
        assert!(out.starts_with("<h1>My Title</h1>"));
    }

    #[test]
    fn title_not_double_prepended() {
        let body = "<h1>Existing</h1><p>x</p>";
        let out = ensure_title_in_body(body, Some("My Title"));
        assert_eq!(out, body);
    }

    #[test]
    fn html_escapes_unsafe_title_chars() {
        let body = "<p>x</p>";
        let out = ensure_title_in_body(body, Some("A & B <c>"));
        assert!(out.starts_with("<h1>A &amp; B &lt;c&gt;</h1>"));
    }
}
