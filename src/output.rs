use crate::cli::Cli;
use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use std::{collections::HashMap, path::PathBuf, str};
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
        let contents = Self::read_files(cli.paths).await.unwrap();
        let mut paths_with_counts = HashMap::new();

        let mut enabled_options = vec![];
        cli.bytes.then(|| enabled_options.push("Bytes"));
        cli.chars.then(|| enabled_options.push("Chars"));
        cli.words.then(|| enabled_options.push("Words"));
        cli.lines.then(|| enabled_options.push("Lines"));
        cli.longest_line
            .then(|| enabled_options.push("Longest line"));

        println!("Calculating the output...\n");
        let mut contents = stream::iter(contents);
        while let Some((path, content)) = contents.next().await {
            let counts = vec![
                cli.bytes.then_some(content.len()),
                cli.chars.then_some(content.chars().count()),
                cli.words.then_some(content.split_whitespace().count()),
                cli.lines.then_some(content.lines().count()),
                cli.longest_line
                    .then_some(content.lines().map(|x| x.len()).max().unwrap_or(0)),
            ]
            .into_iter()
            .flatten()
            .collect();
            paths_with_counts.insert(path, counts);
        }
        Self {
            paths_with_counts,
            enabled_options,
        }
    }

    pub async fn print(self) {
        let titles = {
            let titles = self
                .enabled_options
                .into_iter()
                .map(|x| cell!(Fybi -> x))
                .collect();
            let mut titles = Row::new(titles);
            titles.insert_cell(0, cell!(Fybi -> "Path"));
            titles
        };
        let table = {
            let mut table = Table::new();
            table.set_titles(titles);
            let mut paths_with_counts = stream::iter(self.paths_with_counts);
            while let Some((path, counts)) = paths_with_counts.next().await {
                let counts = counts.into_iter().map(|x| cell!(x)).collect();
                let mut row = Row::new(counts);
                row.insert_cell(0, cell!(Fmb -> path.display()));
                table.add_row(row);
            }
            table.set_format(*FORMAT_BOX_CHARS);
            table
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
            let file = File::open(&path)
                .await
                .context(format!("Unable to open: {}", path.display()))?;
            let metadata = file.metadata().await?;
            let size = metadata.len();
            let bar = ProgressBar::new(size)
                .with_message(format! {"Reading {}", path.display()})
                .with_style(style.clone());
            let bar = bars.add(bar);

            let mut content = String::new();
            let mut reader = BufReader::new(file);
            let mut buf = [0; 40 * 512];
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
