use clap::Parser;
use rwc::{calc, cli::Cli, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let pretty_table = calc::create(cli)?.to_pretty_table();
    pretty_table.printstd();
    Ok(())
}
