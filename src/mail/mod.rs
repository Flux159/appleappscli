//! Mail.app integration.

mod list_recent;
mod mailboxes;
mod send;

pub use list_recent::list_recent;
pub use mailboxes::list_mailboxes;
pub use send::{SendOptions, send};

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum MailCommand {
    /// Send an email via Mail.app.
    Send {
        /// Recipient email address. Repeatable.
        #[arg(long, required = true)]
        to: Vec<String>,

        /// CC recipient(s).
        #[arg(long)]
        cc: Vec<String>,

        /// BCC recipient(s).
        #[arg(long)]
        bcc: Vec<String>,

        /// Subject line.
        #[arg(long)]
        subject: String,

        /// Plain-text body.
        #[arg(long)]
        body: String,

        /// Account name to send from (matches Mail.app account display name).
        /// If omitted, Mail.app picks the default account.
        #[arg(long)]
        from_account: Option<String>,
    },

    /// List mailboxes across all accounts. Output: `<account>\t<mailbox>`.
    ListMailboxes,

    /// List recent inbox messages (newest first).
    ListRecent {
        /// Number of messages to return.
        #[arg(long, default_value_t = 25)]
        limit: u32,
    },
}

pub fn run(cmd: &MailCommand) -> Result<()> {
    match cmd {
        MailCommand::Send {
            to,
            cc,
            bcc,
            subject,
            body,
            from_account,
        } => {
            let opts = SendOptions {
                to: to.clone(),
                cc: cc.clone(),
                bcc: bcc.clone(),
                subject: subject.clone(),
                body: body.clone(),
                from_account: from_account.clone(),
            };
            send(&opts)
        }
        MailCommand::ListMailboxes => {
            for line in list_mailboxes()? {
                println!("{line}");
            }
            Ok(())
        }
        MailCommand::ListRecent { limit } => {
            for line in list_recent(*limit)? {
                println!("{line}");
            }
            Ok(())
        }
    }
}
