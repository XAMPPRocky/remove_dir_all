use std::io;
use std::path::Path;

#[cfg(windows)]
use crate::fs::_remove_dir_contents;

#[cfg(not(windows))]
use crate::unix::_remove_dir_contents;

/// Deletes the contents of `dir_path`, but not the directory iteself.
///
/// If `dir_path` is a symlink to a directory, deletes the contents
/// of that directory.  Fails if `dir_path` does not exist.
pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    // This wrapper function exists because the core function
    // for Windows, in crate::fs, returns a PathBuf, which our
    // caller shouldn't see.
    _remove_dir_contents(path)?;
    Ok(())
}
