mod cli;
mod output;

use crate::cli::Cli;
use crate::output::Output;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = Output::new(cli).await;
    output.print().await;
    Ok(())
}
