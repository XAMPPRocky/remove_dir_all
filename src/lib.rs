//! Reliably remove a directory and all of its children.
//!
//! This library provides a reliable implementation of `remove_dir_all` for
//! Windows. For Unix systems, it re-exports `std::fs::remove_dir_all`.
//!
//! It also provides `remove_dir_contents` and `ensure_empty_dir` for both Unix
//! and Windows.
//!
//! The crate has one feature, enabled by default: "parallel". When this is
//! enabled, deletion of directories proceeds in parallel. And yes, this is a
//! win. On the other hand some users may value build time or code size over
//! tree removal performance. Disable default features and a single threaded
//! implementation is used instead, still with the reliability and contention
//! avoiding features. To support others making this choice, when adding
//! tempfile as a dependency to a library crate, use a feature "parallel" and
//! add it with `default-features = false`. This will permit the user of your
//! library to control the parallel feature inside remove_dir_all : which they
//! may need to do if they also depend on remove_dir_all, and don't want to
//! build things twice with feature-resolver-version-2.
//!
//! e.g.
//!
//! ```Cargo.toml
//! [features]
//! default = ["parallel"]
//! parallel = ["remove_dir_all/parallel"]
//! ...
//! [dependencies]
//! remove_dir_all = {version = "0.7", default-features = false}
//! ```

#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
// See under "known problems" https://rust-lang.github.io/rust-clippy/master/index.html#mutex_atomic
#![allow(clippy::mutex_atomic)]

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doctest!("../README.md");

#[cfg(windows)]
mod fs;

#[cfg(not(windows))]
mod unix;

mod portable;

#[cfg(windows)]
pub use self::fs::remove_dir_all;

#[cfg(not(windows))]
pub use std::fs::remove_dir_all;

pub use portable::ensure_empty_dir;
pub use portable::remove_dir_contents;
