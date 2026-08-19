use anyhow::{Ok, Result};
use std::collections::HashSet;
use std::fs::read;

use seeker::table::parser;

#[test]
fn parse_header_test() -> Result<()> {
    let bytes = read("tests/data/sl/header.json")?;
    let header = parser::parse_header(&bytes)?;

    let bytes = read("tests/data/sl/score.json")?;
    let data = parser::parse_data(&bytes)?;
    let levels: HashSet<_> = data.iter().map(|row| row.level.clone()).collect();
    let levels: Vec<_> = levels.into_iter().collect();

    assert_eq!(header.name, "Satellite");
    assert_eq!(header.symbol, "sl");
    assert_eq!(levels.len(), 13);
    assert_eq!(header.course.unwrap()[0].len(), 13);

    Ok(())
}
