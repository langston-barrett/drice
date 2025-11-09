# Dr. Ice

Dr. Ice diagnoses internal compiler errors (ICEs) in rustc.

In `ice/`, there are Rust programs that cause ICEs on nightly rustc. They
have names like `NNN.rs`, where `NNN` is the number of an open issue number on
[`rust-lang/rust`].

[`rust-lang/rust`]: https://github.com/rust-lang/rust

`scripts/build.sh` runs `rustc` on these programs and collects the output to
`ice/NNN.out`. These outputs are then embedded into `drice`.

To check if a new program---say, `test.rs`---is just a reproduction of a known
ICE, `drice` runs `rustc` on the new program and compares the output against the
known errors. It currently compares them in several ways:

- The error message
- The source line
- The query stack

## Usage

To check if a program is a known ICE:

```sh
drice check test.rs
```

To extract a MCVE from issue `rust-lang/rust#NNNN`:

```sh
drice extract NNNN
```

This will attempt to download the content of the first `rust` Markdown code
block, check if it ICEs with current nightly, ensure that it is not a known
duplicate, and save it to `ice/`.

## Legal

Some files in `ices/` are from `tests/crashes` in `rust-lang/rust`, the licenses
of those files are available under `legal/`.
