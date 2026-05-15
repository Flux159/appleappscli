//! Apple Notes integration.

mod create;
mod list;

pub use create::{CreateOptions, create_note};
pub use list::list_notes;

use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum NotesCommand {
    /// Create a new note from a markdown file (or stdin).
    Create {
        /// Path to a markdown file to convert and use as the note body.
        #[arg(long)]
        markdown_file: Option<PathBuf>,

        /// Raw HTML body. Mutually exclusive with --markdown-file and --stdin.
        #[arg(long, conflicts_with_all = ["markdown_file", "stdin"])]
        html_body: Option<String>,

        /// Read markdown content from stdin.
        #[arg(long, conflicts_with_all = ["markdown_file", "html_body"])]
        stdin: bool,

        /// Folder to place the note in. Created if it doesn't exist.
        #[arg(long)]
        folder: Option<String>,

        /// Optional explicit note title. If omitted, the first H1/H2 of the markdown is used,
        /// or the markdown file's stem.
        #[arg(long)]
        title: Option<String>,
    },

    /// List notes (optionally filtered by folder).
    List {
        /// Folder name to filter by.
        #[arg(long)]
        folder: Option<String>,
    },
}

pub fn run(cmd: &NotesCommand) -> Result<()> {
    match cmd {
        NotesCommand::Create {
            markdown_file,
            html_body,
            stdin,
            folder,
            title,
        } => {
            let opts = CreateOptions {
                markdown_file: markdown_file.clone(),
                html_body: html_body.clone(),
                stdin: *stdin,
                folder: folder.clone(),
                title: title.clone(),
            };
            let id = create_note(&opts)?;
            println!("{id}");
            Ok(())
        }
        NotesCommand::List { folder } => {
            let lines = list_notes(folder.as_deref())?;
            for line in lines {
                println!("{line}");
            }
            Ok(())
        }
    }
}
