# rwc
This is just a simple version of GNU/wc command implementation, written in Rust.  
For rust beginners, feel free to look this project for practice

## Features
what I had archived are as follows:
- Async
- Multiple files support
- Pretty tabled output
- Colorful progress bar for reading files (It is useful when reading large file)
- Completions for common shells [(Look here for more information)](/completions/)

For the Chinese rustaceans, you could view [my blog](https://jedsek.xyz/posts/rust-clap/guide) about using clap-rs to write this project  
Welcome to open issues :)

## Build

### Manually build

```bash
git clone https://github.com/jedsek/rust-wc
cd rust-wc
cargo build --release
```

The path of command is `rust-wc/target/release/rwc`, you could copy/move it to other place  
or just `cargo run --release -- <args>`

### Use release
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

rwc 0.1.0
Author: Jedsek <jedsek@qq.com>

This is just a simple GNU/wc command implementation, written in Rust
It could print <FILE>'s count of bytes, chars, words, lines, and the longest line/word's length

USAGE:
    rwc <--bytes|--chars|--words|--lines|--longest-line> <FILE>...

ARGS:
    <FILE>...    The path(s) you should provide

OPTIONS:
    -b, --bytes           Print the count of bytes
    -c, --chars           Print the count of chars
    -w, --words           Print the count of words
    -l, --lines           Print the count of lines
    -L, --longest-line    Print the length of the longest line
    -h, --help            Print help information
    -V, --version         Print version information
```
