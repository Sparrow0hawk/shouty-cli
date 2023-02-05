# Shouty-cli

This is an example repository for a lecture on git branching.

It's based on the generic example of a CLI app in the [lexopt](https://docs.rs/lexopt/latest/lexopt/) crate documentation.

## Setup

To install this tool you'll need to install [Rust](https://www.rust-lang.org/). 
I'd recommend doing this with [rustup](https://rustup.rs/) which installs the full toolchain.

Once you've installed the Rust toolchain you can clone this repository and install it with `cargo`.

```bash
$ git clone https://github.com/Sparrow0hawk/shouty-cli.git

$ cd shouty-cli

$ cargo run -- --help
Usage: shouty [--shout] THING
```
