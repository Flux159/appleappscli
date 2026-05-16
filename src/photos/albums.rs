use crate::photos::photosdb;
use anyhow::Result;

/// List all user-created album names from Photos.sqlite. One name per line.
pub fn list_albums() -> Result<Vec<String>> {
    let conn = photosdb::open()?;
    photosdb::list_user_albums(&conn)
}
