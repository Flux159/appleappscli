//! Calendar app integration. Stub for future implementation.
//!
//! Planned subcommands:
//! - `aacli calendar add` — create an event
//! - `aacli calendar list` — list upcoming events
//! - `aacli calendar today` — list today's events

use anyhow::{Result, bail};

pub fn run(_args: &[String]) -> Result<()> {
    bail!("calendar subcommand is not implemented yet")
}
