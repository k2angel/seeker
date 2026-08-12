use anyhow::Result;
use std::collections::HashMap;

use crate::model;

pub fn song_map(rows: Vec<model::SongRow>) -> Result<HashMap<i64, model::Song>> {
    rows.into_iter()
        .map(|row| {
            let id = row.id;
            let song = model::Song::try_from(row)?;
            Ok((id, song))
        })
        .collect()
}
