//! Calendar.app integration.

mod add;
mod datetime;
mod list_calendars;
mod list_day;

pub use add::{AddOptions, add_event};
pub use list_calendars::list_calendars;
pub use list_day::list_day;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum CalendarCommand {
    /// Create a new event.
    Add {
        /// Calendar name to add the event to (e.g. "Home", "Work").
        #[arg(long)]
        calendar: String,

        /// Event title / summary.
        #[arg(long)]
        title: String,

        /// Start datetime: `YYYY-MM-DD HH:MM` (24h) or `YYYY-MM-DD` (defaults 00:00).
        #[arg(long)]
        start: String,

        /// End datetime: same format as --start.
        #[arg(long)]
        end: String,

        /// Optional event description / notes.
        #[arg(long)]
        notes: Option<String>,

        /// Optional location string.
        #[arg(long)]
        location: Option<String>,
    },

    /// List events on a specific date.
    ListDay {
        /// Date in `YYYY-MM-DD` format.
        #[arg(long)]
        date: String,

        /// Optional calendar name filter.
        #[arg(long)]
        calendar: Option<String>,
    },

    /// List available calendars.
    ListCalendars,
}

pub fn run(cmd: &CalendarCommand) -> Result<()> {
    match cmd {
        CalendarCommand::Add {
            calendar,
            title,
            start,
            end,
            notes,
            location,
        } => {
            let opts = AddOptions {
                calendar: calendar.clone(),
                title: title.clone(),
                start: start.clone(),
                end: end.clone(),
                notes: notes.clone(),
                location: location.clone(),
            };
            let id = add_event(&opts)?;
            println!("{id}");
            Ok(())
        }
        CalendarCommand::ListDay { date, calendar } => {
            let lines = list_day(date, calendar.as_deref())?;
            for line in lines {
                println!("{line}");
            }
            Ok(())
        }
        CalendarCommand::ListCalendars => {
            for name in list_calendars()? {
                println!("{name}");
            }
            Ok(())
        }
    }
}
