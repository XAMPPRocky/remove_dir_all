//! Private trait to deal with OS variance

#[cfg(not(windows))]
use std::fmt::Debug;
use std::{fs::File, io, path::Path};

pub(crate) trait Io {
    #[cfg(not(windows))]
    type UniqueIdentifier: PartialEq + Debug;

    fn duplicate_fd(f: &mut File) -> io::Result<File>;

    fn open_dir(p: &Path) -> io::Result<File>;

    #[cfg(not(windows))]
    #[allow(dead_code)]
    fn unique_identifier(d: &File) -> io::Result<Self::UniqueIdentifier>;

    /// Returns true if the error from `open_dir_at` indicates the entry is not
    /// a directory (e.g. symlink, FIFO, socket, regular file) and should be
    /// removed with `unlink_at` instead.
    #[cfg(not(windows))]
    fn is_not_dir_open_error(e: &io::Error) -> bool;
}
