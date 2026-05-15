//! Photos app integration. Stub for future implementation.
//!
//! Planned subcommands:
//! - `aacli photos import --path ...`
//! - `aacli photos albums`
//! - `aacli photos export --album ... --to ...`

use anyhow::{Result, bail};

pub fn run(_args: &[String]) -> Result<()> {
    bail!("photos subcommand is not implemented yet")
}
