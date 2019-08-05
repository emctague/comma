# comma

[![Crates.io](https://img.shields.io/crates/v/comma?style=flat-square)](https://crates.io/crates/comma)
[![docs.rs](https://docs.rs/comma/badge.svg)](https://docs.rs/comma)
[![Build Status](https://travis-ci.org/emctague/comma.svg?branch=master)](https://travis-ci.org/emctague/comma)

`comma` parses shell-style commands, e.g. `sendmsg joe "I say \"hi\" to you!"`, into a simple structure with a `name`
and a list of `arguments`. It collapses excess whitespace, and allows for quoting or backslash-escaping text.

## Cargo

```toml
[dependencies]
comma = "0.1.1"
```

## Usage

```rust
use comma::Command;

fn main () {
    let parsed = Command::from_str("sendmsg joe \"I say \\"hi\\" to you!\"");
    println!("Command name: {}", parsed.name); // Command name: sendmsg
    println!("Command arguments: {:#?}", parsed.arguments); // Command arguments: [ "joe", "I say \"hi\" to you!" ]
}
```