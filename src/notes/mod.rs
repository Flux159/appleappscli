//! Apple Notes integration.

mod append;
mod attach;
mod create;
mod list;
mod move_note;
mod read;

pub use append::{AppendOptions, append_note};
pub use attach::attach_image;
pub use create::{CreateOptions, create_note};
pub use list::list_notes;
pub use move_note::move_note;
pub use read::read_note;

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

    /// Append content to an existing note by id.
    Append {
        /// Note id (e.g. `x-coredata://...`).
        #[arg(long)]
        id: String,

        /// Markdown file to append (converted to HTML).
        #[arg(long)]
        markdown_file: Option<PathBuf>,

        /// Raw HTML to append.
        #[arg(long, conflicts_with_all = ["markdown_file", "stdin"])]
        html_body: Option<String>,

        /// Read markdown content from stdin.
        #[arg(long, conflicts_with_all = ["markdown_file", "html_body"])]
        stdin: bool,
    },

    /// Read the HTML body of a note by id.
    Read {
        /// Note id (e.g. `x-coredata://...`).
        #[arg(long)]
        id: String,
    },

    /// Move a note to a different folder.
    Move {
        /// Note id (e.g. `x-coredata://...`).
        #[arg(long)]
        id: String,

        /// Destination folder name. Created if it doesn't exist.
        #[arg(long)]
        folder: String,
    },

    /// Attach a local image file to an existing note.
    Attach {
        /// Note id (e.g. `x-coredata://...`).
        #[arg(long)]
        id: String,

        /// Path to image file (png, jpg, gif, heic, etc.).
        #[arg(long)]
        image: PathBuf,
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
        NotesCommand::Append {
            id,
            markdown_file,
            html_body,
            stdin,
        } => {
            let opts = AppendOptions {
                id: id.clone(),
                markdown_file: markdown_file.clone(),
                html_body: html_body.clone(),
                stdin: *stdin,
            };
            append_note(&opts)
        }
        NotesCommand::Read { id } => {
            let body = read_note(id)?;
            print!("{body}");
            Ok(())
        }
        NotesCommand::Move { id, folder } => move_note(id, folder),
        NotesCommand::Attach { id, image } => attach_image(id, image),
    }
}
