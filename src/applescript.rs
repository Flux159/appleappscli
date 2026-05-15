use anyhow::{Result, anyhow};
use std::process::Command;

/// Run an AppleScript and return stdout on success.
pub fn run(script: &str) -> Result<String> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| anyhow!("failed to invoke osascript: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("osascript failed: {}", stderr.trim()));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Escape a string for safe inclusion inside an AppleScript double-quoted string literal.
///
/// AppleScript string literals treat `"` and `\` specially. Embedding HTML
/// (which contains `"` in attribute values) directly into a script will break it.
/// This function escapes those characters.
pub fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
