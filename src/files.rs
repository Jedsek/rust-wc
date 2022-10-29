#![allow(unused)]

use crate::Result;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::process;
use std::sync::atomic::AtomicUsize;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::Mutex,
};

type PathWithContent = HashMap<PathBuf, String>;
static INPUTTED_FILE_NUMBER: AtomicUsize = AtomicUsize::new(0);
const BUFFER_SIZR: usize = 16 * 1024;

trait PathExt {
    fn without_dotted_prefix(&self) -> bool;
    fn add_dotted_prefix(&mut self);
}

impl PathExt for PathBuf {
    fn without_dotted_prefix(&self) -> bool {
        self.is_relative() && !self.starts_with("../") && !self.starts_with("./")
    }

    fn add_dotted_prefix(&mut self) {
        *self = PathBuf::from_iter([OsStr::new("./"), self.as_os_str()]);
    }
}

pub fn read_files(paths: Vec<PathBuf>) -> Result<PathWithContent> {
    println!("Reading files / Getting content from stdin:");

    let num = Mutex::new(-1);
    let result = paths
        .into_par_iter()
        .filter(|path| path.is_file() || path.as_os_str() == "-")
        .map(|mut path| {
            let should_read_from_input = path.as_os_str() == "-";

            let content = get_content(&path, should_read_from_input);

            if path.without_dotted_prefix() {
                path.add_dotted_prefix();
            }

            if should_read_from_input {
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
        .collect();
    Ok(result)
}

fn get_content(path: &PathBuf, should_read_from_input: bool) -> Result<String> {
    if should_read_from_input {
        read_from_stdin()
    } else {
        let bars = MultiProgress::new();
        let style =
            ProgressStyle::with_template("[{elapsed}][{percent}%] {bar:45.cyan/blue} {bytes} {wide_msg}")?
                .progress_chars(">-");
        read_file_with_progress(path, style, bars)
    }
}

fn read_file_with_progress(path: &PathBuf, style: ProgressStyle, bars: MultiProgress) -> Result<String> {
    let mut content = String::new();

    let file = File::open(path)?;
    let size = file.metadata()?.len();

    let bar = ProgressBar::new(size).with_message(format! {"Reading {}", path.display()}).with_style(style);
    let bar = bars.add(bar);

    let mut bufreader = BufReader::new(file);
    let mut buf = [0; BUFFER_SIZR];

    while let Ok(n) = bufreader.read(&mut buf) && n != 0 {
        bar.inc(n as u64);
        content += &String::from_utf8_lossy(&buf[..n]);
    }
    bar.finish_with_message("Done!");

    Ok(content)
}

fn read_from_stdin() -> Result<String> {
    let mut content = vec![];
    std::io::stdin().read_to_end(&mut content)?;
    Ok(String::from_utf8(content)?)
}
