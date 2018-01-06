# Rust library: tty-read-rs
A Rust library to read raw real-time input from the terminal.

This crate provides a reader, that reads raw input to the terminal as soon as
the user presses a key. This is useful for processing raw and/or real-time
input data.
For this reader, the user isn't required to press the enter/return key to
pass the input to the application, as this is normally the case when reading
directly from `stdin`.

This crate puts the terminal in a raw mode when a reader is opened.
This might cause weird formatting behaviour when printing to `stdout`, until
the reader is dropped. You may want to consider to lock `stdin` until then.
When the reader is dropped, the terminal state is automatically reverted.

**Note:** This crate is a proposal. This library hasn't been released yet in
the Rust crates index. Feel free however to use it in it's current state.

## Features
- Read raw input from the tty.
- Process input in the tty as soon as keys are pressed.
- Cleans up terminal state after use.

## Requirements
- Linux, macOS or BSD (Windows is not supported).
- Native `libc` library.

## Example
Here is a minimal example on how to use this crate.

Add to `Cargo.toml`:  
```toml
[dependencies]
tty-read = { git = "https://github.com/timvisee/tty-read-rs" }
```

Example code in `main.rs`:
```rust
extern crate tty_read;

use tty_read::{ReaderOptions, TermReader};

fn main() {
    // Configure reader options
    let options = ReaderOptions::default();

    // Open a reader
    let reader = TermReader::open_stdin(&options)
        .expect("failed to open stdin reader");

    // Read 5 bytes
    let input = reader.read_bytes(5)
        .expect("failed to read input bytes");

    // Print the result
    println!("Received bytes: {:?}", input);
}
```

## TODO
- Configurable properties:
  - Print input characters or not.
  - Cancel input when pressing CTRL+C or not, catch these by default.
  - Input timeout.
  - Lock `stdin` and/or `stdout`.
- Support for Windows.
- Input prompt (persistent or not).
- Don't flush `stdin` when opening a reader.
- Additional reading functions.
- ...

## License
This project is released under the MIT license.
Check out the [LICENSE](LICENSE) file for more information.
