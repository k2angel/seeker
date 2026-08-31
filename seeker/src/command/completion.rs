use anyhow::{Ok, Result};
use clap::CommandFactory;
use clap_complete::aot::{Shell, generate};

use crate::Cli;

pub fn run(shell: Shell) -> Result<()> {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "seeker", &mut std::io::stdout());

    Ok(())
}
