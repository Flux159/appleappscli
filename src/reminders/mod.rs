//! Apple Reminders integration.

mod complete;
mod create;
mod delete;
mod list;

pub use complete::complete_reminder;
pub use create::{CreateOptions, create_reminder};
pub use delete::delete_reminder;
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

    /// Mark a reminder as completed by id.
    Complete {
        /// Reminder id (e.g. `x-apple-reminderkit://...`).
        #[arg(long)]
        id: String,
    },

    /// Delete a reminder by id.
    Delete {
        /// Reminder id (e.g. `x-apple-reminderkit://...`).
        #[arg(long)]
        id: String,
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
        RemindersCommand::Complete { id } => complete_reminder(id),
        RemindersCommand::Delete { id } => delete_reminder(id),
    }
}
