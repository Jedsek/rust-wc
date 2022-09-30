use crate::{cli::Cli, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use partial_application::partial;
use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use std::{collections::HashMap, path::PathBuf, str};
use tap::prelude::*;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tokio_stream::{self as stream, StreamExt};

pub struct Output {
    paths_with_counts: HashMap<PathBuf, Vec<usize>>,
    enabled_options: Vec<&'static str>,
}

impl Output {
    pub async fn new(cli: Cli) -> Self {
        fn calculate(cli: &Cli, content: String) -> Vec<usize> {
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
        let calculate = partial!(calculate => &cli, _);

        let mut paths_with_counts = HashMap::new();
        let mut enabled_options = vec![];

        cli.bytes.then(|| enabled_options.push("Bytes"));
        cli.chars.then(|| enabled_options.push("Chars"));
        cli.words.then(|| enabled_options.push("Words"));
        cli.lines.then(|| enabled_options.push("Lines"));
        cli.longest_line
            .then(|| enabled_options.push("Longest line"));

        println!("Calculating...\n");
        let mut contents = cli
            .paths
            .clone()
            .pipe(Self::read_files)
            .await
            .unwrap()
            .pipe(stream::iter);

        while let Some((path, content)) = contents.next().await {
            let counts = calculate(content);
            paths_with_counts.insert(path, counts);
        }
        Self {
            paths_with_counts,
            enabled_options,
        }
    }

    pub async fn print(self) {
        let titles = self
            .enabled_options
            .into_iter()
            .map(|x| cell!(Fybi -> x))
            .collect::<Vec<_>>()
            .pipe(Row::new)
            .tap_mut(|x| x.insert_cell(0, cell!(Fybi -> "Path")));

        let table = {
            let mut table = Table::new().tap_mut(|x| x.set_titles(titles));
            let mut paths_with_counts = stream::iter(self.paths_with_counts);
            while let Some((path, counts)) = paths_with_counts.next().await {
                let row = counts
                    .into_iter()
                    .map(|x| cell!(x))
                    .collect::<Vec<_>>()
                    .pipe(Row::new)
                    .tap_mut(|x| x.insert_cell(0, cell!(Fmb -> path.display())));
                table.add_row(row);
            }
            table.tap_mut(|x| x.set_format(*FORMAT_BOX_CHARS))
        };
        table.printstd();
    }

    async fn read_files(paths: Vec<PathBuf>) -> Result<HashMap<PathBuf, String>> {
        let bars = MultiProgress::new();
        let style = ProgressStyle::with_template(
            "[{elapsed}][{percent}%] {bar:45.cyan/blue} {bytes} {wide_msg}",
        )?
        .progress_chars(">-");
        let mut contents = HashMap::new();

        bars.println("Reading files...")?;
        let mut paths = stream::iter(paths);
        while let Some(path) = paths.next().await {
            let file = File::open(&path).await?;
            let size = file.metadata().await?.len();

            let bar = ProgressBar::new(size)
                .with_message(format! {"Reading {}", path.display()})
                .with_style(style.clone())
                .pipe(|x| bars.add(x));

            let mut content = String::new();
            let mut reader = BufReader::new(file);
            let mut buf = [0; 100 * 512];
            while let Ok(n) = reader.read(&mut buf).await && n != 0 {
                bar.inc(n as u64);
                content += str::from_utf8(&buf[..n])?;
            }
            bar.finish_with_message("Done!");
            contents.insert(path, content);
        }
        Ok(contents)
    }
}
