use anyhow::Result;
use appleappscli::{calendar, mail, messages, notes, photos, reminders, terminal};
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
    /// Messages.app operations (send via iMessage; list/read via chat.db — requires Full Disk Access).
    Messages {
        #[command(subcommand)]
        cmd: messages::MessagesCommand,
    },
    /// Photos.app operations.
    Photos {
        #[command(subcommand)]
        cmd: photos::PhotosCommand,
    },
    /// Terminal.app operations.
    Terminal {
        #[command(subcommand)]
        cmd: terminal::TerminalCommand,
    },
    /// Mail.app operations.
    Mail {
        #[command(subcommand)]
        cmd: mail::MailCommand,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Command::Notes { cmd } => notes::run(cmd),
        Command::Reminders { cmd } => reminders::run(cmd),
        Command::Calendar { cmd } => calendar::run(cmd),
        Command::Messages { cmd } => messages::run(cmd),
        Command::Photos { cmd } => photos::run(cmd),
        Command::Terminal { cmd } => terminal::run(cmd),
        Command::Mail { cmd } => mail::run(cmd),
    }
}
