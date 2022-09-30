#![feature(let_chains)]

pub mod cli;
pub mod output;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;