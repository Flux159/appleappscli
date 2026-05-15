//! Apple Reminders integration.

mod create;
mod list;

pub use create::{CreateOptions, create_reminder};
pub use list::list_reminders;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RemindersCommand {
    /// Create a new reminder.
    Create {
        /// Title / body of the reminder.
        title: String,

        /// Due date/time in `YYYY-MM-DD HH:MM` (24h) format. Optional.
        #[arg(long)]
        due: Option<String>,

        /// Optional notes / body text.
        #[arg(long)]
        notes: Option<String>,

        /// Reminders list to add to. Defaults to the default list.
        #[arg(long)]
        list: Option<String>,
    },

    /// List reminders (optionally from a specific list).
    List {
        /// Reminders list name.
        #[arg(long)]
        list: Option<String>,

        /// Include completed reminders.
        #[arg(long)]
        all: bool,
    },
}

pub fn run(cmd: &RemindersCommand) -> Result<()> {
    match cmd {
        RemindersCommand::Create {
            title,
            due,
            notes,
            list,
        } => {
            let opts = CreateOptions {
                title: title.clone(),
                due: due.clone(),
                notes: notes.clone(),
                list: list.clone(),
            };
            let id = create_reminder(&opts)?;
            println!("{id}");
            Ok(())
        }
        RemindersCommand::List { list, all } => {
            let lines = list_reminders(list.as_deref(), *all)?;
            for line in lines {
                println!("{line}");
            }
            Ok(())
        }
    }
}
