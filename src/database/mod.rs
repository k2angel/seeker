mod chart;
mod song;
mod table;

use anyhow::Result;
use std::path::Path;

use crate::model::{SearchExpr, SearchTerm};

// const VERSION: i32 = 1;
const SCHEMA: &str = include_str!("schema.sql");

pub struct Database {
    conn: rusqlite::Connection,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = rusqlite::Connection::open(path)?;

        Ok(Self { conn })
    }

    pub fn create_schema(&self) -> Result<()> {
        self.conn.execute_batch(SCHEMA)?;

        Ok(())
    }

    pub fn transaction(&mut self) -> Result<rusqlite::Transaction<'_>> {
        Ok(self.conn.transaction()?)
    }
}

fn search_term_condition(
    term: &SearchTerm,
    all_fields: &[&str],
    params: &mut Vec<String>,
) -> String {
    params.push(format!("%{}%", term.value));
    let n = params.len();

    let condition = match term.field.as_deref() {
        None => all_fields
            .iter()
            .map(|field| format!("{field} LIKE ?{n}"))
            .collect::<Vec<_>>()
            .join(" OR "),

        Some("artist") if all_fields.contains(&"sub_artist") => format!(
            "artist || CASE WHEN sub_artist IS NOT NULL THEN ' / ' || sub_artist ELSE '' END LIKE ?{n}"
        ),

        Some("title") if all_fields.contains(&"subtitle") => format!(
            "title || CASE WHEN subtitle IS NOT NULL THEN ' ' || subtitle ELSE '' END LIKE ?{n}"
        ),

        Some(field) if all_fields.contains(&field) => {
            format!("{field} LIKE ?{n}")
        }

        _ => "1 = 0".to_owned(),
    };

    format!("({condition})")
}

fn search_condition(expr: &SearchExpr, all_fields: &[&str], params: &mut Vec<String>) -> String {
    let groups = match expr {
        SearchExpr::And(terms) => vec![terms],
        SearchExpr::Or(groups) => groups.iter().collect(),
    };

    groups
        .into_iter()
        .map(|terms| {
            let conditions = terms
                .iter()
                .map(|term| search_term_condition(term, all_fields, params))
                .collect::<Vec<_>>();

            format!("({})", conditions.join(" AND "))
        })
        .collect::<Vec<_>>()
        .join(" OR ")
}
