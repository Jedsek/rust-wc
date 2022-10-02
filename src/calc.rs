use crate::{
    cli::{Cli, SubCommands},
    Result,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use prettytable::{cell, format::consts::FORMAT_BOX_CHARS, Row, Table};
use rayon::prelude::*;
use std::{
    ffi::OsString,
    fs::File,
    io::{BufReader, Read},
    process,
    sync::Mutex,
    {collections::HashMap, path::PathBuf, str},
};
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
    let contents = cli.paths.clone().pipe(read_files)?;

    println!("Calculating...");
    let paths_with_counts = contents
        .into_par_iter()
        .map(|(path, content)| (path, get_new_counts(&cli, content)))
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
    cli.longest_line.then(|| enabled_options.push("Maximum line width (Bytes)"));

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

fn read_files(paths: Vec<PathBuf>) -> Result<HashMap<PathBuf, String>> {
    let bars = MultiProgress::new();
    let style =
        ProgressStyle::with_template("[{elapsed}][{percent}%] {bar:45.cyan/blue} {bytes} {wide_msg}")?
            .progress_chars(">-");

    bars.println("Reading files / Getting content from stdin:")?;

    let num = Mutex::new(-1);
    let result = paths
        .into_par_iter()
        .filter(|path| path.is_file() || path.as_os_str() == "-")
        .map(|mut path| {
            let should_input = path.as_os_str() == "-";

            let content = if !should_input {
                read_file_with_progress(&path, &style, &bars)
            } else {
                content_from_stdin().tap(|_| println!())
            };

            if path.is_relative() && !path.starts_with("../") && !path.starts_with("./") {
                path = PathBuf::from_iter([OsString::from("./"), path.into_os_string()]);
            }

            if should_input {
                let mut num = num.lock().unwrap();
                *num += 1;
                path = PathBuf::from(format!("Input/{}", num));
            }

            let content = content.unwrap_or_else(|err| {
                eprintln!("{}: {}", path.display(), err);
                process::exit(1);
            });

            (path, content)
        })
        .collect::<HashMap<_, _>>();
    Ok(result)
}

fn get_new_counts(cli: &Cli, content: String) -> Vec<usize> {
    let v: Vec<Option<usize>> = vec![None; 5];
    v.into_par_iter()
        .enumerate()
        .map(|(idx, _)| match idx {
            0 => cli.bytes.then_some(content.len()),
            1 => cli.chars.then_some(content.chars().count()),
            2 => cli.words.then_some(content.split_whitespace().count()),
            3 => cli.lines.then_some(content.lines().count()),
            4 => cli.longest_line.then_some(content.lines().map(|x| x.len()).max().unwrap_or(0)),
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn read_file_with_progress(path: &PathBuf, style: &ProgressStyle, bars: &MultiProgress) -> Result<String> {
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
        content += &String::from_utf8_lossy(&buf[..n]);
    }
    bar.finish_with_message("Done!");

    Ok(content)
}

fn content_from_stdin() -> Result<String> {
    let mut content = vec![];
    std::io::stdin().read_to_end(&mut content)?;
    Ok(String::from_utf8(content)?)
}
