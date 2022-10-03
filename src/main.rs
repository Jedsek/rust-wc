use clap::Parser;
use lib::{calc, cli::Cli, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    calc::create(cli)?.print();
    Ok(())
}
