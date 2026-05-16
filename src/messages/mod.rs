//! Messages.app integration.
//!
//! Send uses AppleScript. List and Read query `~/Library/Messages/chat.db` directly,
//! which requires **Full Disk Access** for the calling terminal.

mod chatdb;
mod list;
mod read;
mod send;

pub use list::list;
pub use read::read;
pub use send::send_message;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum MessagesCommand {
    /// Send a message via iMessage (falls back to SMS if recipient is not on iMessage).
    Send {
        /// Recipient — phone number (E.164 preferred), email, or chat handle.
        #[arg(long)]
        to: String,

        /// Message text to send.
        #[arg(long)]
        text: String,
    },

    /// List chats sorted by most-recent message (matches Messages.app UI order).
    /// Requires Full Disk Access for the terminal.
    List {
        /// Number of chats to return.
        #[arg(long, default_value_t = 25)]
        limit: u32,
    },

    /// Read recent messages from a chat.
    /// Requires Full Disk Access for the terminal.
    Read {
        /// Chat identifier — phone number, email, or chat guid (from `messages list`).
        #[arg(long)]
        chat: String,

        /// Number of recent messages to return.
        #[arg(long, default_value_t = 25)]
        limit: u32,
    },
}

pub fn run(cmd: &MessagesCommand) -> Result<()> {
    match cmd {
        MessagesCommand::Send { to, text } => send_message(to, text),
        MessagesCommand::List { limit } => {
            for line in list(*limit)? {
                println!("{line}");
            }
            Ok(())
        }
        MessagesCommand::Read { chat, limit } => {
            for line in read(chat, *limit)? {
                println!("{line}");
            }
            Ok(())
        }
    }
}
