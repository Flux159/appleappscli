//! Terminal.app integration.

mod list_windows;
mod new_tab;
mod new_window;
mod send;

pub use list_windows::list_windows;
pub use new_tab::new_tab;
pub use new_window::new_window;
pub use send::send_command;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum TerminalCommand {
    /// Open a new Terminal window, optionally running a command.
    NewWindow {
        /// Command to run in the new window.
        #[arg(long)]
        command: Option<String>,
    },

    /// Open a new tab in the front Terminal window (or a new window if none).
    NewTab {
        /// Command to run in the new tab.
        #[arg(long)]
        command: Option<String>,
    },

    /// Send a command to a Terminal window/tab (defaults to front window's selected tab).
    Send {
        /// Window id (from `terminal list-windows`).
        #[arg(long)]
        window: Option<String>,

        /// Tab index within the window (1-based).
        #[arg(long)]
        tab: Option<u32>,

        /// Command to run.
        #[arg(long)]
        command: String,
    },

    /// List open Terminal windows. Output: `<window_id>\t<tab_count>\t<title>\t<tty>`.
    ListWindows,
}

pub fn run(cmd: &TerminalCommand) -> Result<()> {
    match cmd {
        TerminalCommand::NewWindow { command } => {
            let id = new_window(command.as_deref())?;
            println!("{id}");
            Ok(())
        }
        TerminalCommand::NewTab { command } => {
            let info = new_tab(command.as_deref())?;
            println!("{info}");
            Ok(())
        }
        TerminalCommand::Send {
            window,
            tab,
            command,
        } => send_command(window.as_deref(), *tab, command),
        TerminalCommand::ListWindows => {
            for line in list_windows()? {
                println!("{line}");
            }
            Ok(())
        }
    }
}
