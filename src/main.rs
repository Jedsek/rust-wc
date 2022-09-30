use clap::Parser;
use rwc::{calc, cli::Cli, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    calc::create(cli)?.print();
    Ok(())
}
