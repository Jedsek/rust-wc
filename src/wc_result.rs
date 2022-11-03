use crate::{
    cli::{Cli, SubCommands},
    files::read_files,
    Counts, Result,
};
use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use rayon::prelude::*;
use std::{collections::HashMap, path::PathBuf, str};

pub struct WcResult {
    enabled_options: Vec<&'static str>,
    paths_with_counts: HashMap<PathBuf, Counts>,
}

pub fn get(mut cli: Cli) -> Result<WcResult> {
    println!("Please waiting...\n");

    match cli.sub_commands {
        Some(SubCommands::All { ref paths }) => {
            cli.paths = paths.clone();
            cli.enable_all_options();
        }
        None => cli.enable_all_options(),
    };

    println!("Calculating...");
    let wc_result = WcResult {
        enabled_options: cli.get_enabled_options(),
        paths_with_counts: {
            let contents = read_files(cli.paths.clone())?;
            contents.into_par_iter().map(|(path, content)| (path, calculate_counts(&cli, content))).collect()
        },
    };

    Ok(wc_result)
}

impl WcResult {
    pub fn to_pretty_table(self) -> Table {
        let titles = {
            let enabled_options = self.enabled_options;
            let mut titles = Row::new(enabled_options.into_iter().map(|x| cell!(Fybi -> x)).collect());
            titles.insert_cell(0, cell!(Fybi -> "Path"));
            titles
        };

        let mut table = Table::new();
        table.set_titles(titles);
        table.set_format(*FORMAT_BOX_CHARS);

        for (path, counts) in self.paths_with_counts {
            let mut row = Row::new(counts.into_iter().map(|x| cell!(x)).collect());
            let path_cell = if path.starts_with("Input") {
                cell!(Fbb -> path.display())
            } else {
                cell!(Fmb -> path.display())
            };

            row.insert_cell(0, path_cell);
            table.add_row(row);
        }

        table
    }
}

fn calculate_counts(cli: &Cli, content: String) -> Counts {
    let v: Vec<Option<usize>> = vec![None; 5];
    v.into_par_iter()
        .enumerate()
        .map(|(idx, _)| match idx {
            0 => cli.bytes.then_some(content.len()),
            1 => cli.chars.then_some(content.chars().count()),
            2 => cli.words.then_some(content.split_whitespace().count()),
            3 => cli.lines.then_some(content.lines().count()),
            4 => cli
                .longest_line
                .then_some(content.lines().map(unicode_width::UnicodeWidthStr::width).max().unwrap_or(0)),
            _ => None,
        })
        .flatten()
        .collect()
}
