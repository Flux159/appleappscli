use super::chatdb::{cocoa_to_unix_seconds, format_local_timestamp, list_chats, open};
use anyhow::Result;

/// List chats sorted by most-recent message (matches Messages.app UI order).
/// Output per line: `<chat_identifier>\t<display_name>\t<last_message_iso>\t<last_message_preview>`
pub fn list(limit: u32) -> Result<Vec<String>> {
    let conn = open()?;
    let chats = list_chats(&conn, limit)?;

    Ok(chats
        .into_iter()
        .map(|c| {
            let unix = cocoa_to_unix_seconds(c.last_message_cocoa);
            let when = format_local_timestamp(unix);
            // Truncate preview to one line, max 80 chars
            let preview = c
                .last_message_text
                .replace(['\n', '\r', '\t'], " ")
                .chars()
                .take(80)
                .collect::<String>();
            let display = if c.display_name.is_empty() {
                "-".to_string()
            } else {
                c.display_name
            };
            format!("{}\t{}\t{}\t{}", c.chat_identifier, display, when, preview)
        })
        .collect())
}
