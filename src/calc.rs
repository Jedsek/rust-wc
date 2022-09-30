use crate::{cli::Cli, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process;
use std::{collections::HashMap, path::PathBuf, str};
use tap::prelude::*;

pub struct Output {
    paths_with_counts: HashMap<PathBuf, Vec<usize>>,
    enabled_options: Vec<&'static str>,
}

pub fn create(cli: Cli) -> Result<Output> {
    println!("Please waiting...\n");
    let contents = cli.paths.clone().pipe(read_files)?;

    let paths_with_counts = contents
        .into_par_iter()
        .map(|(path, content)| (path, get_counts(&cli, content)))
        .collect::<HashMap<_, _>>();

    let enabled_options = {
        let mut enabled_options = vec![];
        cli.bytes.then(|| enabled_options.push("Bytes"));
        cli.chars.then(|| enabled_options.push("Chars"));
        cli.words.then(|| enabled_options.push("Words"));
        cli.lines.then(|| enabled_options.push("Lines"));
        cli.longest_line
            .then(|| enabled_options.push("Longest line"));
        enabled_options
    };

    Ok(Output {
        paths_with_counts,
        enabled_options,
    })
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
            self.paths_with_counts
                .into_iter()
                .for_each(|(path, counts)| {
                    let row = counts
                        .into_iter()
                        .map(|x| cell!(x))
                        .collect::<Vec<_>>()
                        .pipe(Row::new)
                        .tap_mut(|x| x.insert_cell(0, cell!(Fmb -> path.display())));
                    table.add_row(row);
                });
            table.tap_mut(|x| x.set_format(*FORMAT_BOX_CHARS))
        };
        table.printstd();
    }
}

fn read_files(paths: Vec<PathBuf>) -> Result<HashMap<PathBuf, String>> {
    let bars = MultiProgress::new();
    let style = ProgressStyle::with_template(
        "[{elapsed}][{percent}%] {bar:45.cyan/blue} {bytes} {wide_msg}",
    )?
    .progress_chars(">-");

    bars.println("Reading files...")?;
    Ok(paths
        .into_par_iter()
        .filter(|path| path.is_file())
        .map(|path| {
            let content = read_file(&path, &style, &bars).unwrap_or_else(|err| {
                eprintln!("{}: {}", path.display(), err);
                process::exit(1);
            });
            (path, content)
        })
        .collect::<HashMap<_, _>>())
}

fn get_counts(cli: &Cli, content: String) -> Vec<usize> {
    vec![
        cli.bytes.then_some(content.len()),
        cli.chars.then_some(content.chars().count()),
        cli.words.then_some(content.split_whitespace().count()),
        cli.lines.then_some(content.lines().count()),
        cli.longest_line
            .then_some(content.lines().map(|x| x.len()).max().unwrap_or(0)),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn read_file(path: &PathBuf, style: &ProgressStyle, bars: &MultiProgress) -> Result<String> {
    let mut content = String::new();

    let file = File::open(path)?;
    let size = file.metadata()?.len();
    let bar = ProgressBar::new(size)
        .with_message(format! {"Reading {}", path.display()})
        .with_style(style.clone())
        .pipe(|x| bars.add(x));

    let mut reader = BufReader::new(file);
    let mut buf = [0; 200 * 512];

    while let Ok(n) = reader.read(&mut buf) && n != 0 {
        bar.inc(n as u64);
        content += str::from_utf8(&buf[..n])?;
    }
    bar.finish_with_message("Done!");

    Ok(content)
}
