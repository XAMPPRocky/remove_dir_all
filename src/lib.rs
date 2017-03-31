extern crate winapi;
extern crate kernel32 as kernel;

#[cfg(windows)]
mod fs;

#[cfg(windows)]
pub use self::fs::remove_dir_all;

#[cfg(not(windows))]
pub use std::fs::remove_dir_all;

