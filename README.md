# remove_dir_all

[**documentation**](https://docs.rs/remove_dir_all/)

A reliable implementation of `remove_dir_all` for Windows. For Unix systems
re-exports `std::fs::remove_dir_all`.

```rust
extern crate remove_dir_all;

use remove_dir_all::*;

fn main() {
    remove_dir_all("./temp/").unwrap();
}
```
