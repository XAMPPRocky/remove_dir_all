//! Private trait to deal with OS variance

use std::{fmt::Debug, fs::File, io, path::Path};

pub(crate) trait Io {
    type UniqueIdentifier: PartialEq + Debug;

    fn duplicate_fd(f: &mut File) -> io::Result<File>;

    fn open_dir(p: &Path) -> io::Result<File>;
    fn unique_identifier(d: &File) -> io::Result<Self::UniqueIdentifier>;

    fn is_eloop(e: &io::Error) -> bool;
}
