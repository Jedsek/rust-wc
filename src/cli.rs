use std::path::PathBuf;
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(
    author = "Author: Jedsek <jedsek@qq.com>",
    version,
    // global_setting = AppSettings::DeriveDisplayOrder,
    group(ArgGroup::new("options").multiple(true).required(true).args(&[ "bytes", "chars", "words", "lines", "longest_line"])),
    about = 
r#"
A simple GNU/wc command implementation written in Rust, which could print <FILE>'s bytes, chars, words, lines, and more options...
The output will be formatted as a colorful table :)"#,
)]
pub struct Cli {
    /// The path(s) you should provide
    #[arg(value_name = "FILE", required = true)]
    pub paths: Vec<PathBuf>,

    /// Show the count of bytes
    #[arg(short, long)]
    pub bytes: bool,

    /// Show the count of chars
    #[arg(short, long)]
    pub chars: bool,

    /// Show the count of words
    #[arg(short, long)]
    pub words: bool,

    /// Show the count of lines
    #[arg(short, long)]
    pub lines: bool,

    /// Show the length of the longest line
    #[arg(short = 'L', long)]
    pub longest_line: bool,
}
