use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;
use stripmargin::StripMargin;

#[derive(Parser)]
#[command(
    author, version, about, subcommand_negates_reqs = true,
    group(
        ArgGroup::new("options")
            .multiple(true)
            .required(true)
            .args(&[ "bytes", "chars", "words", "lines", "longest_line"])
    ),
)]
pub struct Cli {
    #[arg(value_parser = check_path, value_name = "PATH", default_value = "-", help = {r#"The path(s) you should provide
        |Note when without FILE or it is `-`, read standard input (stop inputting by `CTRL-D`)
        |The file read from stdin will prefix with `Input/`, and the other will prefix with `./`"#.strip_margin()}
    )]
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

    /// Print the maximum line width (Unicode)
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
        #[arg(value_parser = check_path, value_name = "PATH", default_value = "-")]
        paths: Vec<PathBuf>,
    },
}

fn check_path(filename: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(filename);
    if filename == "-" || path.exists() {
        Ok(path)
    } else {
        Err(format!("No such path: `{}`", path.display()))
    }
}

impl Cli {
    pub fn enable_all_options(&mut self) {
        self.bytes = true;
        self.chars = true;
        self.words = true;
        self.lines = true;
        self.longest_line = true;
    }
    pub fn get_enabled_options(&self) -> Vec<&'static str> {
        let mut enabled_options = vec![];

        self.bytes.then(|| enabled_options.push("Bytes"));
        self.chars.then(|| enabled_options.push("Chars"));
        self.words.then(|| enabled_options.push("Words"));
        self.lines.then(|| enabled_options.push("Lines"));
        self.longest_line.then(|| enabled_options.push("Maximum line width (Unicode)"));

        enabled_options
    }
}
