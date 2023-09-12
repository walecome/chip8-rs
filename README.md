# Simple Chip 8 interpreter in Rust

## Get the code and run things

``` sh
git clone git@github.com:walecome/chip8-rs.git
cd chip8-rs
cargo run -- --rom <path to ROM file>
```

## Usage
Using the `cargo run` command is the simplest way to run the interpreter. ROMs (both for testing or gaming) can be found online. By default, the interpreter aims to run 700 instructions per second, this can be changed with the `--instructions-per-second` command line flag. The display (and timers) aim to update 60 times per second. Some of the quirks that vary between interpreter implementations can be configured using command line flags (`cargo run -- --help`).
