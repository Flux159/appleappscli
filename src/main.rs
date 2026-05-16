use anyhow::Result;
use appleappscli::{calendar, messages, notes, photos, reminders, terminal};
use clap::{Parser, Subcommand};

/// CLI for scripting macOS apps via AppleScript.
#[derive(Parser, Debug)]
#[command(name = "aacli", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Apple Notes operations.
    Notes {
        #[command(subcommand)]
        cmd: notes::NotesCommand,
    },
    /// Apple Reminders operations.
    Reminders {
        #[command(subcommand)]
        cmd: reminders::RemindersCommand,
    },
    /// Calendar.app operations.
    Calendar {
        #[command(subcommand)]
        cmd: calendar::CalendarCommand,
    },
    /// Messages (stub).
    Messages {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Photos (stub).
    Photos {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Terminal (stub).
    Terminal {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Notes { cmd } => notes::run(cmd),
        Command::Reminders { cmd } => reminders::run(cmd),
        Command::Calendar { cmd } => calendar::run(cmd),
        Command::Messages { args } => messages::run(args),
        Command::Photos { args } => photos::run(args),
        Command::Terminal { args } => terminal::run(args),
    }
}
