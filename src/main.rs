use clap::Parser;
use rust_wc::{cli::Cli, wc_result, Result};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let pretty_table = wc_result::get(cli)?.to_pretty_table();
    pretty_table.printstd();
    Ok(())
}
