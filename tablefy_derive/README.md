# tablefy_derive

[![tablefy_derive](https://img.shields.io/crates/v/tablefy_derive.svg)](https://crates.io/crates/tablefy_derive)
[![tablefy_derive](https://docs.rs/tablefy_derive/badge.svg)](https://docs.rs/crate/tablefy_derive)

A procedural macro to derive the `Tablefy` trait for any struct.
_(Currently with the limitation that said struct needs to have values that implement the `Display` trait.)_

[Changelog to keep track of progress](CHANGELOG.md)

## `#[derive(Tablefy)]`

The main `derive` macro! Derives the `Tablefy` trait for any struct satisfying the before-mentioned limitation _(for now)_.
The field names are set as the headers, with the contents being set as the rows. However, if you want to change the name of
the headers, then you should use...

## `#[header(name = <str>)]`

Put this above any field in order to customize what its header name should be. Note that `<str>` can be _any_ valid string 
literal. If any cryptic errors arise from using this however, feel free to raise an issue.