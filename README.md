# rwc
This is just a simple version of GNU/wc command clone, written in Rust.  
For rust beginners, feel free to look this project for practice :)  
(Using Clap-v4)  

[![asciicast](https://asciinema.org/a/534647.svg)](https://asciinema.org/a/534647)

## Features
What I had archived are as follows:
- Support for reading multiple files and calculating the result in parrllel
- Pretty tabled output
- Colorful progress bar for reading files (It is useful when reading large file)
- Completions for common shells [(Look here for more information)](/completions/)

**Note:**  
It is super faster and prettier than `wc` command when reading a large of big files. (Thanks to `rayon`)  
But it may be slower when reading a small numbers of small files due to time spending for runtime building.

For the Chinese, you could view [my blog](https://jedsek.xyz/posts/rust-clap/intro) about using clap-rs to write this project  

## Installation

### Manual

```bash
git clone https://github.com/jedsek/rust-wc
cd rust-wc
cargo build --release
```

The path of command is `rust-wc/target/release/rwc`, you could copy/move it to other place  
or just `cargo run --release -- <args>`

### Release
You could download the binary published in [Release](https://github.com/Jedsek/rust-wc/releases/)

## Examples

- Count the files in the specified directory with all options enabled:

```
$ rwc all src/*
Please waiting...

Reading files / Getting content from stdin:
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 153B Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 126B Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 1.65 KiB Done!
[0s][100%] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 5.43 KiB Done!
Calculating...
┌───────────────┬───────┬───────┬───────┬───────┬──────────────────────────────┐
│ Path          │ Bytes │ Chars │ Words │ Lines │ Maximum line width (Unicode) │
├───────────────┼───────┼───────┼───────┼───────┼──────────────────────────────┤
│ ./src/calc.rs │ 5562  │ 5562  │ 450   │ 181   │ 110                          │
├───────────────┼───────┼───────┼───────┼───────┼──────────────────────────────┤
│ ./src/main.rs │ 153   │ 153   │ 18    │ 8     │ 34                           │
├───────────────┼───────┼───────┼───────┼───────┼──────────────────────────────┤
│ ./src/cli.rs  │ 1694  │ 1694  │ 198   │ 63    │ 127                          │
├───────────────┼───────┼───────┼───────┼───────┼──────────────────────────────┤
│ ./src/lib.rs  │ 126   │ 126   │ 14    │ 6     │ 72                           │
└───────────────┴───────┴───────┴───────┴───────┴──────────────────────────────┘
```

- Get help:

```
$ rwc -h
A GNU/wc clone written in rust, which is faster when reading a large of big files

Usage: rwc <--bytes|--chars|--words|--lines|--longest-line> [PATH]...
       rwc [PATH]... <COMMAND>

Commands:
  all   Enabled all available options
  help  Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]...  The path(s) you should provide
             Note when without FILE or it is `-`, read standard input (stop inputting by `CTRL-D`)
             The file read from stdin will prefix with `Input/`, and the other will prefix with `./` [default: -]

Options:
  -b, --bytes         Print the byte counts
  -c, --chars         Print the character counts
  -w, --words         Print the word counts
  -l, --lines         Print the line counts
  -L, --longest-line  Print the maximum line width (Unicode)
  -h, --help          Print help information
  -V, --version       Print version information
```

## Welcome
Welcome to open issus when you meet problems or want to improve the code.  
Thanks :)

## TODOs
- [ ] Better handling directories arguments
- [ ] Output should report errors when the file could't be read
- [ ] Respect `.gitignore` file
