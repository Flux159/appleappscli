//! Messages app integration. Stub for future implementation.
//!
//! Planned subcommands:
//! - `aacli messages send --to "+1..." --text "..."`
//! - `aacli messages list-recent`

use anyhow::{Result, bail};

pub fn run(_args: &[String]) -> Result<()> {
    bail!("messages subcommand is not implemented yet")
}
