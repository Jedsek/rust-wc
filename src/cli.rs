use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    author = "Author: Jedsek <jedsek@qq.com>",
    version,
    // global_setting = AppSettings::DeriveDisplayOrder,
    group(ArgGroup::new("options").multiple(true).required(true).args(&[ "bytes", "chars", "words", "lines", "longest_line"])),
    subcommand_negates_reqs = true,
    about = 
r#"A simple GNU/wc command clone, written in Rust

It could count file's bytes, chars, words and more...
The output will be formatted as a colorful table :)"#,
)]
pub struct Cli {
    /// The path(s) you should provide
    #[arg(value_parser = check_path, value_name = "PATH", required = true)]
    pub paths: Vec<PathBuf>,

    /// Print the byte counts
    #[arg(short, long)]
    pub bytes: bool,

    /// Print the character counts
    #[arg(short, long)]
    pub chars: bool,

    /// Print the word counts
    #[arg(short, long)]
    pub words: bool,

    /// Print the line counts
    #[arg(short, long)]
    pub lines: bool,

    /// Print the maximum line width (Bytes)
    #[arg(short = 'L', long)]
    pub longest_line: bool,

    #[command(subcommand)]
    pub sub_commands: Option<SubCommands>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Enabled all available options
    All {
        /// The path(s) you should provide
        #[arg(value_parser = check_path, value_name = "PATH", required = true)]
        paths: Vec<PathBuf>,
    },
}

fn check_path(path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path);
    if path.exists() || path.as_os_str() == "-" {
        Ok(path)
    } else {
        Err(format!("No such path: `{}`", path.display()))
    }
}
