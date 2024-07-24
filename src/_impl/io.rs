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
    fn unique_identifier(d: &File) -> io::Result<Self::UniqueIdentifier>;

    #[cfg(not(windows))]
    fn is_eloop(e: &io::Error) -> bool;
}
