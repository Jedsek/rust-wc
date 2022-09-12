use clap::Parser;
use rwc::{cli::Cli, output::Output, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = Output::new(cli).await;
    output.print().await;
    Ok(())
}
