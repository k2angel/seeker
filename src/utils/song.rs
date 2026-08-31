use anyhow::Result;
use std::collections::HashMap;

use crate::model::SongRow;

pub fn song_map(rows: Vec<SongRow>) -> Result<HashMap<i64, SongRow>> {
    rows.into_iter()
        .map(|row| {
            let id = row.id;
            Ok((id, row))
        })
        .collect()
}
