use anyhow::Result;
use std::collections::HashSet;

use crate::database::Database;
use crate::model::table::Table;
use crate::table::parser;
use crate::utils;

pub fn load_table(db: &mut Database, url: &str) -> Result<Table> {
    let header_url = parser::get_header_url(url)?;

    let header_bytes = parser::get_header(header_url.as_ref())?;
    let header = parser::parse_header(header_bytes.as_ref())?;
    let result = db.get_table(&header.name)?;

    let data_url = header_url.join(&header.data_url)?;

    let (bytes, etag, last_modified) = match &result {
        Some(row) => parser::get_data(data_url.as_ref(), row.etag.as_deref(), row.last_modified)?,
        None => parser::get_data(data_url.as_ref(), None, None)?,
    };

    let Some(bytes) = bytes else {
        return Table::try_from(result.unwrap());
    };

    let level_order = match header.level_order {
        Some(order) => order,
        None => {
            let data = parser::parse_data(bytes.as_ref())?;

            let mut numbers: Vec<u64> = data
                .iter()
                .filter_map(|row| row.level.parse::<u64>().ok())
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            numbers.sort();
            numbers.into_iter().map(|num| num.to_string()).collect()
        }
    };

    let table = Table {
        header_url: header_url.to_string(),
        data_url: data_url.to_string(),
        name: header.name,
        symbol: header.symbol,
        level_order,
        course: header.course,
        sha256: utils::sha256sum(bytes.as_ref()),
        etag,
        last_modified,
        data: bytes.to_vec(),
    };

    match &result {
        Some(row) if row.sha256 != table.sha256 => db.update_table(row.id, &table)?,
        Some(_) => {}
        None => db.insert_table(&table)?,
    }

    Ok(table)
}
