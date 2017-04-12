Rust Syntax Ast Builder
=======================
[![Build Status](https://travis-ci.org/serde-rs/aster.svg?branch=master)](https://travis-ci.org/serde-rs/aster)
[![Latest Version](https://img.shields.io/crates/v/aster.svg)](https://crates.io/crates/aster)

Aster is a framework that simplifies generating Rust AST.
Example
-------

This example demonstrates how to use Aster to create a simple compound expression.
Let's start with the

`Cargo.toml` file:

```toml
[package]
name = "hello_world"
version = "0.3.0"
authors = ["Erick Tryzelaar <erick.tryzelaar@gmail.com>"]

[features]
nightly = []

[dependencies]
aster = { version = "*", default_features = false }
```

Here is the actual script:

```rust
#![feature(rustc_private)]

extern crate aster;
extern crate syntax;

fn main() {
    let builder = aster::AstBuilder::new();

    let expr = builder.expr()
        .add().u32(1).u32(2);

    // prints `1 + 2`.
    println!("{}", syntax::print::pprust::expr_to_string(&expr));
}
```
