use std::fs;
use std::io;
use std::path::Path;

#[cfg(not(target_os = "macos"))]
fn remove_file_or_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match fs::remove_file(&path) {
        // Unfortunately, there is no ErrorKind for EISDIR
        Err(e) if e.raw_os_error() == Some(libc::EISDIR) =>
            fs::remove_dir_all(&path),
        r => r,
    }
}

#[cfg(target_os = "macos")]
fn remove_file_or_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    match fs::remove_file(&path) {
        // On Macos trying to unlink a directory results in EPERM
        Err(e) if e.raw_os_error() == Some(libc::EPERM)
        && fs::metadata(&path)?.is_dir() =>
            fs::remove_dir_all(&path),
        r => r,
    }
}

pub fn _remove_dir_contents<P: AsRef<Path>>(path: P) -> Result<(), io::Error> {
    for entry in fs::read_dir(path)? {
        let entry_path = entry?.path();
        remove_file_or_dir_all(&entry_path)?;
    }

    Ok(())
}
