`rest_easy` lint
================

[![Build Status](https://travis-ci.org/cmr/rest_easy.png?branch=master)](https://travis-ci.org/cmr/rest_easy)

Tells you when `rustc` has finished doing everything that could cause
compilation failure, so you can go back to looking at cat pictures while it
does codegen.

Use it
======

Add to your `Cargo.toml`:
```toml
[dependencies.rest_easy]
git = "https://github.com/cmr/rest_easy"
```

Add to your `main.rs` or `lib.rs`:

```
#![feature(phase)]

#[phase(plugin)] extern crate rest_easy
```
