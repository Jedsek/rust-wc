# rwc
This is just a simple version of GNU/wc command implementation, written in Rust.  
For rust beginners, feel free to look this project for practice :)

## Features
What I had archived are as follows:
- Support for reading multiple files in parrllel
- Pretty tabled output
- Colorful progress bar for reading files (It is useful when reading large file)
- Completions for common shells [(Look here for more information)](/completions/)

For the Chinese rustaceans, you could view [my blog](https://jedsek.xyz/posts/rust-clap/guide) about using clap-rs to write this project  

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
You could download the binary file published in [release-site](https://github.com/Jedsek/rust-wc/releases/)

## Examples

```bash
$ rwc src/cli.rs -bcwlL

┌────────────┬───────┬───────┬───────┬───────┬──────────────┐
│ Path       │ Bytes │ Chars │ Words │ Lines │ Longest line │
├────────────┼───────┼───────┼───────┼───────┼──────────────┤
│ src/cli.rs │ 1212  │ 1212  │ 144   │ 41    │ 127          │
└────────────┴───────┴───────┴───────┴───────┴──────────────┘
```

```bash
$ rwc -h
A simple GNU/wc command implementation written in Rust, which could print <FILE>'s bytes, chars, words and more...
The output will be formatted as a colorful table :)

Usage: rwc <--bytes|--chars|--words|--lines|--longest-line> <FILE>...

Arguments:
  <FILE>...  The path(s) you should provide

Options:
  -b, --bytes         Show the count of bytes
  -c, --chars         Show the count of chars
  -w, --words         Show the count of words
  -l, --lines         Show the count of lines
  -L, --longest-line  Show the length of the longest line
  -h, --help          Print help information
  -V, --version       Print version information
```
