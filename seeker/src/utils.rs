use anyhow::{Result, bail};
use dialoguer::MultiSelect;
use std::io::Write;

pub fn confirm_input(command: &str, items: &[String]) -> Result<Vec<usize>> {
    match items.len() {
        0 => bail!("No matching items found."),
        1 => {
            print!("Really {} 1 items from the library? (Yes/no) > ", command);
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "n" | "no" => bail!("Cancelled"),
                _ => Ok(vec![0]),
            }
        }
        count => {
            print!(
                "Really {} {} items from the library? (yes/no/Select) > ",
                command, count
            );
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().to_ascii_lowercase().as_str() {
                "y" | "yes" => Ok((0..count).collect()),
                "n" | "no" => bail!("Cancelled"),
                _ => {
                    let selections = MultiSelect::new()
                        .with_prompt("Select items")
                        .items(items)
                        .interact()?;

                    Ok(selections)
                }
            }
        }
    }
}
