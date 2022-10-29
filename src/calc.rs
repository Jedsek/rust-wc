use crate::{
    cli::{Cli, SubCommands},
    file::read_files,
    Result,
};

use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use rayon::prelude::*;
use std::{collections::HashMap, path::PathBuf, str};
use tap::prelude::*;

pub struct Output {
    paths_with_counts: HashMap<PathBuf, Vec<usize>>,
    enabled_options: Vec<&'static str>,
}

pub fn create(mut cli: Cli) -> Result<Output> {
    println!("Please waiting...\n");
    if let Some(SubCommands::All { ref paths }) = cli.sub_commands {
        cli.bytes = true;
        cli.chars = true;
        cli.words = true;
        cli.lines = true;
        cli.longest_line = true;

        cli.paths = paths.clone();
    }
    let contents = read_files(cli.paths.clone())?;

    println!("Calculating...");
    let paths_with_counts = contents
        .into_par_iter()
        .map(|(path, content)| (path, calculate_count(&cli, content)))
        .collect::<HashMap<_, _>>();

    let enabled_options = get_enabled_options(&cli);

    let output = Output {
        paths_with_counts,
        enabled_options,
    };

    Ok(output)
}

fn get_enabled_options(cli: &Cli) -> Vec<&'static str> {
    let mut enabled_options = vec![];

    cli.bytes.then(|| enabled_options.push("Bytes"));
    cli.chars.then(|| enabled_options.push("Chars"));
    cli.words.then(|| enabled_options.push("Words"));
    cli.lines.then(|| enabled_options.push("Lines"));
    cli.longest_line.then(|| enabled_options.push("Maximum line width (Chars)"));

    enabled_options
}

impl Output {
    pub fn print(self) {
        let titles = self
            .enabled_options
            .into_iter()
            .map(|x| cell!(Fybi -> x))
            .collect::<Vec<_>>()
            .pipe(Row::new)
            .tap_mut(|x| x.insert_cell(0, cell!(Fybi -> "Path")));

        let table = {
            let mut table = Table::new().tap_mut(|x| x.set_titles(titles));
            table.set_format(*FORMAT_BOX_CHARS);

            self.paths_with_counts.into_iter().for_each(|(path, counts)| {
                let row = counts.into_iter().map(|x| cell!(x)).collect::<Vec<_>>();
                let row = Row::new(row).tap_mut(|x| {
                    let cell = if path.starts_with("Input") {
                        cell!(Fbb -> path.display())
                    } else {
                        cell!(Fmb -> path.display())
                    };
                    x.insert_cell(0, cell);
                });
                table.add_row(row);
            });

            table
        };
        table.printstd();
    }
}

fn calculate_count(cli: &Cli, content: String) -> Vec<usize> {
    let v: Vec<Option<usize>> = vec![None; 5];
    v.into_par_iter()
        .enumerate()
        .map(|(idx, _)| match idx {
            0 => cli.bytes.then_some(content.len()),
            1 => cli.chars.then_some(content.chars().count()),
            2 => cli.words.then_some(content.split_whitespace().count()),
            3 => cli.lines.then_some(content.lines().count()),
            4 => cli.longest_line.then_some(content.lines().map(|x| x.chars().count()).max().unwrap_or(0)),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>()
}
