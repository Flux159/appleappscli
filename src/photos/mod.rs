//! Photos.app integration.

mod albums;
mod export;
mod find;

pub use albums::list_albums;
pub use export::{ExportFormat, export_photo};
pub use find::{PhotoInfo, Query, find};

use anyhow::{Result, anyhow};
use clap::Subcommand;
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum PhotosCommand {
    /// List all album names.
    Albums,

    /// Find photo(s) by filename, library index, or stable id.
    ///
    /// Exactly one of --name / --index / --id must be supplied.
    Find {
        /// Filename substring (e.g. "IMG_0595" matches IMG_0595.HEIC).
        #[arg(long, conflicts_with_all = ["index", "id"])]
        name: Option<String>,

        /// 1-based position in the library (matches "N of M" position in Photos).
        #[arg(long, conflicts_with_all = ["name", "id"])]
        index: Option<u64>,

        /// Stable photo id (UUID-like string from a prior `find` call).
        #[arg(long, conflicts_with_all = ["name", "index"])]
        id: Option<String>,

        /// Max number of results for --name lookups.
        #[arg(long, default_value_t = 10)]
        limit: u32,
    },

    /// Export a photo to a directory, optionally converting to PNG or JPG.
    Export {
        /// Stable photo id (preferred — get one from `photos find`).
        #[arg(long, conflicts_with_all = ["name", "index"])]
        id: Option<String>,

        /// Filename substring (uses first match).
        #[arg(long, conflicts_with_all = ["id", "index"])]
        name: Option<String>,

        /// 1-based library index.
        #[arg(long, conflicts_with_all = ["id", "name"])]
        index: Option<u64>,

        /// Output directory (created if missing).
        #[arg(long)]
        output_dir: PathBuf,

        /// Output format: original, png, or jpg.
        #[arg(long, value_enum, default_value_t = ExportFormat::Original)]
        format: ExportFormat,
    },
}

pub fn run(cmd: &PhotosCommand) -> Result<()> {
    match cmd {
        PhotosCommand::Albums => {
            for a in list_albums()? {
                println!("{a}");
            }
            Ok(())
        }
        PhotosCommand::Find {
            name,
            index,
            id,
            limit,
        } => {
            let query = build_query(name.clone(), *index, id.clone())?;
            let hits = find(&query, *limit)?;
            for p in hits {
                println!(
                    "{}\t{}\t{}\t{}x{}",
                    p.id, p.filename, p.date, p.width, p.height
                );
            }
            Ok(())
        }
        PhotosCommand::Export {
            id,
            name,
            index,
            output_dir,
            format,
        } => {
            // Resolve to a concrete id first.
            let resolved_id = if let Some(i) = id {
                i.clone()
            } else {
                let query = build_query(name.clone(), *index, None)?;
                let hits = find(&query, 1)?;
                hits.into_iter()
                    .next()
                    .ok_or_else(|| anyhow!("no photo matched the query"))?
                    .id
            };
            let path = export_photo(&resolved_id, output_dir, *format)?;
            println!("{}", path.display());
            Ok(())
        }
    }
}

fn build_query(name: Option<String>, index: Option<u64>, id: Option<String>) -> Result<Query> {
    match (name, index, id) {
        (Some(n), None, None) => Ok(Query::Name(n)),
        (None, Some(i), None) => Ok(Query::Index(i)),
        (None, None, Some(i)) => Ok(Query::Id(i)),
        _ => Err(anyhow!(
            "exactly one of --name / --index / --id must be supplied"
        )),
    }
}
