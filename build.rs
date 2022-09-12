use clap::CommandFactory;
use clap_complete::{generate_to, shells::*};
use std::io::Result;

include!("src/cli.rs");

fn main() -> Result<()> {
    let outdir = "completions";
    let app_name = "rwc";
    let mut cmd = Cli::command();

    generate_to(Bash, &mut cmd, app_name, outdir)?;
    generate_to(Zsh, &mut cmd, app_name, outdir)?;
    generate_to(Fish, &mut cmd, app_name, outdir)?;
    generate_to(Elvish, &mut cmd, app_name, outdir)?;
    generate_to(PowerShell, &mut cmd, app_name, outdir)?;

    Ok(())
}
