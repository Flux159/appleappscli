use crate::photos::photosdb::{self, PhotoRow};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct PhotoInfo {
    pub id: String,
    pub filename: String,
    pub date: String,
    pub width: i64,
    pub height: i64,
}

impl From<PhotoRow> for PhotoInfo {
    fn from(r: PhotoRow) -> Self {
        PhotoInfo {
            id: r.uuid,
            filename: r.original_filename,
            date: r.date_local,
            width: r.width,
            height: r.height,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Query {
    /// Filename substring match against ZADDITIONALASSETATTRIBUTES.ZORIGINALFILENAME
    /// (e.g. "IMG_0595" matches IMG_0595.HEIC, IMG_0595_edit.jpg).
    Name(String),
    /// 1-based index in the library, oldest-first (chronological).
    Index(u64),
    /// Stable photo UUID (from a prior `find` call).
    Id(String),
}

pub fn find(query: &Query, limit: u32) -> Result<Vec<PhotoInfo>> {
    let conn = photosdb::open()?;
    let rows = match query {
        Query::Name(n) => photosdb::find_by_name(&conn, n, limit)?,
        Query::Index(i) => photosdb::find_by_index(&conn, *i)?.into_iter().collect(),
        Query::Id(id) => photosdb::find_by_uuid(&conn, id)?.into_iter().collect(),
    };
    Ok(rows.into_iter().map(PhotoInfo::from).collect())
}
