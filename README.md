# rwc
This is just a simple version of GNU/wc command clone, written in Rust.  
For rust beginners, feel free to look this project for practice :)

(Using Clap-v4)

## Features
What I had archived are as follows:
- Support for reading multiple files and calculating the count in parrllel
- Pretty tabled output
- Colorful progress bar for reading files (It is useful when reading large file)
- Completions for common shells [(Look here for more information)](/completions/)

**Note:**  
It is super faster and prettier than `wc` command when reading a large of big files. (Thanks to `rayon`)  
To put it another word, it is slower when reading a small numbers of small files.

For the Chinese, you could view [my blog](https://jedsek.xyz/posts/rust-clap/guide) about using clap-rs to write this project  

## Build

### Manual

```bash
git clone https://github.com/jedsek/rust-wc
cd rust-wc
cargo build --release
```

The path of command is `rust-wc/target/release/rwc`, you could copy/move it to other place  
or just `cargo run --release -- <args>`

### Release

From crates.io:

```bash
cargo install rust-wc
```

And you could also get the binary file in [Release](https://github.com/Jedsek/rust-wc/releases/)



## Examples

- Count the files in the specified directory with all options enabled:

```bash
$ rwc all src/*
Please waiting...

Reading files / Getting content from stdin:
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 153B Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 126B Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 1.65 KiB Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 5.43 KiB Done!
Calculating...
┌───────────────┬───────┬───────┬───────┬───────┬────────────────────────────┐
│ Path          │ Bytes │ Chars │ Words │ Lines │ Maximum line width (Bytes) │
├───────────────┼───────┼───────┼───────┼───────┼────────────────────────────┤
│ ./src/calc.rs │ 5562  │ 5562  │ 450   │ 181   │ 110                        │
├───────────────┼───────┼───────┼───────┼───────┼────────────────────────────┤
│ ./src/main.rs │ 153   │ 153   │ 18    │ 8     │ 34                         │
├───────────────┼───────┼───────┼───────┼───────┼────────────────────────────┤
│ ./src/cli.rs  │ 1694  │ 1694  │ 198   │ 63    │ 127                        │
├───────────────┼───────┼───────┼───────┼───────┼────────────────────────────┤
│ ./src/lib.rs  │ 126   │ 126   │ 14    │ 6     │ 72                         │
└───────────────┴───────┴───────┴───────┴───────┴────────────────────────────┘

```

- Get help:

```bash
$ rwc -h
A simple GNU/wc command clone, written in Rust

It could count file's bytes, chars, words and more...
The output will be formatted as a colorful table :)

Usage: rwc <--bytes|--chars|--words|--lines|--longest-line> <PATH>...
       rwc [PATH]... <COMMAND>

Commands:
  all   Enabled all available options
  help  Print this message or the help of the given subcommand(s)

Arguments:
  <PATH>...  The path(s) you should provide

Options:
  -b, --bytes         Print the byte counts
  -c, --chars         Print the character counts
  -w, --words         Print the word counts
  -l, --lines         Print the line counts
  -L, --longest-line  Print the maximum line width (Bytes)
  -h, --help          Print help information
  -V, --version       Print version information

```
