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

#[cfg(test)]
mod test {
    use tempfile::TempDir;
    use crate::remove_dir_all;
    use crate::remove_dir_contents;
    use std::fs::{self, File};
    use std::io;

    fn expect_failure<T>(k: io::ErrorKind, r: io::Result<T>) -> io::Result<()> {
        match r {
            Err(e) if e.kind() == k => Ok(()),
            Err(e) => Err(e),
            Ok(_) => Err(io::Error::new(
                io::ErrorKind::Other,
                "unexpected success".to_string(),
            )),
        }
    }

    #[test]
    fn mkdir_rm() -> Result<(), io::Error> {
        let tmp = TempDir::new()?;
        let ours = tmp.path().join("t.mkdir");
        let file = ours.join("file");
        fs::create_dir(&ours)?;
        File::create(&file)?;
        File::open(&file)?;

        expect_failure(io::ErrorKind::Other, remove_dir_contents(&file))?;

        remove_dir_contents(&ours)?;
        expect_failure(io::ErrorKind::NotFound, File::open(&file))?;

        remove_dir_contents(&ours)?;
        remove_dir_all(&ours)?;
        expect_failure(io::ErrorKind::NotFound, remove_dir_contents(&ours))?;
        Ok(())
    }
}
