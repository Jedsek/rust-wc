#![feature(let_chains)]

use std::{collections::HashMap, path::PathBuf};

pub mod cli;
pub mod files;
pub mod wc_result;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub type Counts = Vec<usize>;
pub type PathWithContent = HashMap<PathBuf, String>;
