# rwc
This is just a simple version of GNU/wc command implementation, written in Rust.  
For rust beginners, feel free to look this project for practice

## Features
what I had archived are as follows:
- Async
- Multiple files support
- Pretty tabled output
- Colorful progress bar for reading files (It is useful when reading large file)
- Completions for common shells [(Look here for more infomation)](/completions/)

For the Chinese rustaceans, you could view [my blog](https://jedsek.xyz/posts/rust-clap/guide) about using clap-rs to write this project  
Welcome to open issues :)

## Build

### Manually build

```bash
git clone https://github.com/jedsek/rust-wc
cd rust-wc
cargo build --release
```

The path of command is `rust-wc/target/release/rwc`, you could copy/move it to other place, or just `cargo run --release -- <args>`

### Use release



## Examples

```
cargo 
┌────────────┬───────┬───────┬───────┬───────┬──────────────┐
│ Path       │ Bytes │ Chars │ Words │ Lines │ Longest line │
├────────────┼───────┼───────┼───────┼───────┼──────────────┤
│ src/cli.rs │ 1212  │ 1212  │ 144   │ 41    │ 127          │
└────────────┴───────┴───────┴───────┴───────┴──────────────┘
```
