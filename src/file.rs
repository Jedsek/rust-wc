use std::ffi::OsString;
use std::process;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
    sync::Mutex,
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use tap::prelude::*;

use crate::Result;

pub fn read_file_with_progress(
    path: &PathBuf,
    style: &ProgressStyle,
    bars: &MultiProgress,
) -> Result<String> {
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

pub fn content_from_stdin() -> Result<String> {
    let mut content = vec![];
    std::io::stdin().read_to_end(&mut content)?;
    Ok(String::from_utf8(content)?)
}

pub fn read_files(paths: Vec<PathBuf>) -> Result<HashMap<PathBuf, String>> {
    let bars = MultiProgress::new();
    let style =
        ProgressStyle::with_template("[{elapsed}][{percent}%] {bar:45.cyan/blue} {bytes} {wide_msg}")?
            .progress_chars(">-");

    println!("Reading files / Getting content from stdin:");

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
