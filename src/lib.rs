#![feature(let_chains)]

pub mod calc;
pub mod cli;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
