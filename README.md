# remove_dir_all

[![Latest Version](https://img.shields.io/crates/v/remove_dir_all.svg)](https://crates.io/crates/remove_dir_all)
[![Docs](https://docs.rs/remove_dir_all/badge.svg)](https://docs.rs/remove_dir_all)
[![License](https://img.shields.io/crates/l/remove_dir_all.svg)](https://github.com/XAMPPRocky/remove_dir_all)

## Description

A reliable implementation of `remove_dir_all` for Windows. For Unix systems
re-exports `std::fs::remove_dir_all`.

Also provides `remove_dir_contents` for both Windows and Unix.  This
removes just the contents of the directory, if it exists.

And provides `ensure_e pty_dir` for both Windows and Unix.  This
creates an empty directory, or deletes the contents of an existing one.

```rust,no_run
extern crate remove_dir_all;

use remove_dir_all::*;

fn main() {
    remove_dir_all("./temp/").unwrap();
}
```

## Minimum Rust Version
The minimum rust version for `remove_dir_all` is the latest stable release, and the minimum version may be bumped through patch releases. You can pin to a specific version by setting by add `=` to your version (e.g. `=0.6.0`), or commiting a `Cargo.lock` file to your project.
