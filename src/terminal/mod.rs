//! Terminal app integration. Stub for future implementation.
//!
//! Planned subcommands:
//! - `aacli terminal new-tab --command "..."`
//! - `aacli terminal new-window --command "..."`

use anyhow::{Result, bail};

pub fn run(_args: &[String]) -> Result<()> {
    bail!("terminal subcommand is not implemented yet")
}
