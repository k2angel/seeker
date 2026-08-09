mod chart;
mod song;

use anyhow::Result;
use std::path::Path;

use crate::cli::{SearchExpr, SearchField};

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
                .map(|term| {
                    params.push(format!("%{}%", term.value));
                    let n = params.len();

                    match term.field {
                        SearchField::All => {
                            let fields = all_fields
                                .iter()
                                .map(|field| format!("{field} LIKE ?{n}"))
                                .collect::<Vec<_>>();

                            format!("({})", fields.join(" OR "))
                        }

                        SearchField::Artist => {
                            if all_fields.contains(&"sub_artist") {
                                format!("artist || CASE WHEN sub_artist IS NOT NULL THEN ' / ' || sub_artist ELSE '' END LIKE ?{n}")
                            } else {
                                format!("artist LIKE ?{n}")
                            }
                        }

                        SearchField::Title => {
                            if all_fields.contains(&"subtitle") {
                                format!("title || CASE WHEN subtitle IS NOT NULL THEN ' ' || subtitle ELSE '' END LIKE ?{n}")
                            } else {
                                format!("title LIKE ?{n}")
                            }
                        }
                    }
                })
                .collect::<Vec<_>>();

            format!("({})", conditions.join(" AND "))
        })
        .collect::<Vec<_>>()
        .join(" OR ")
}
