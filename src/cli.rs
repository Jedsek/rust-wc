use std::path::PathBuf;

use clap::{AppSettings, ArgGroup, Parser};

#[derive(Parser)]
#[clap(
    author = "Author: Jedsek <jedsek@qq.com>",
    version,
    global_setting = AppSettings::DeriveDisplayOrder,
    group(ArgGroup::new("options").multiple(true).required(true).args(&[ "bytes", "chars", "words", "lines", "longest-line"])),
    about = 
r#"
This is just a simple GNU/wc command implementation, written in Rust
It could print <FILE>'s count of bytes, chars, words, lines, and the longest line/word's length
"#,
)]
pub struct Cli {
    /// The path(s) you should provide
    #[clap(value_parser, value_name = "FILE", required = true)]
    pub paths: Vec<PathBuf>,

    /// Print the count of bytes
    #[clap(value_parser, short, long, action)]
    pub bytes: bool,

    /// Print the count of chars
    #[clap(value_parser, short, long, action)]
    pub chars: bool,

    /// Print the count of words
    #[clap(value_parser, short, long, action)]
    pub words: bool,

    /// Print the count of lines
    #[clap(value_parser, short, long, action)]
    pub lines: bool,

    /// Print the length of the longest line
    #[clap(value_parser, short = 'L', long, action)]
    pub longest_line: bool,
}
