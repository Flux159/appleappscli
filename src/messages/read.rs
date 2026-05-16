use super::chatdb::{cocoa_to_unix_seconds, format_local_timestamp, open, read_messages};
use anyhow::Result;

/// Read recent messages from a chat (identified by phone number, email, or guid).
/// Returns oldest-first within the most-recent N. Output per line:
/// `<iso_timestamp>\t<direction>\t<sender>\t<text>`
/// where direction is `me` or `them`.
pub fn read(chat_key: &str, limit: u32) -> Result<Vec<String>> {
    let conn = open()?;
    let msgs = read_messages(&conn, chat_key, limit)?;

    Ok(msgs
        .into_iter()
        .map(|m| {
            let when = format_local_timestamp(cocoa_to_unix_seconds(m.date_cocoa));
            let direction = if m.is_from_me { "me" } else { "them" };
            let sender = if m.sender.is_empty() {
                "-".to_string()
            } else {
                m.sender
            };
            let text = m.text.replace(['\n', '\r'], " ");
            format!("{when}\t{direction}\t{sender}\t{text}")
        })
        .collect())
}
