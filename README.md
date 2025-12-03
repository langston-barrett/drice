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

## Trophy case

This is a list of duplicate issues on `rust-lang` repos found by Dr. Ice.

- [rust#129425](https://github.com/rust-lang/rust/issues/129425#event-21017164882)

For an issue to make this list, it needs to be discovered by Dr. Ice and then
confirmed to be a duplicate. To be confirmed as a duplicate is to:

- be closed as a duplicate,
- be explicitly mentioned as a duplicate, or
- be closed by the same commit as the older issue.

To be discovered by Dr. Ice means for Dr. Ice to describe it as a duplicate when
it was not previously confirmed to be a duplicate.

## Legal

Some files in `ices/` are from `tests/crashes` in `rust-lang/rust`, the licenses
of those files are available under `legal/`.
