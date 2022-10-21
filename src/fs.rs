use std::{
    fs::{self, OpenOptions},
    io,
    ffi::c_void,
    mem::size_of,
    os::windows::prelude::*,
    path::{Path, PathBuf},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;
use windows_sys::Win32::Storage::FileSystem::*;
use windows_sys::Win32::Foundation::HANDLE;

/// Reliably removes a directory and all of its children.
///
/// ```rust
/// use std::fs;
/// use remove_dir_all::*;
///
/// fs::create_dir("./temp/").unwrap();
//// remove_dir_all("./temp/").unwrap();
/// ```
///
/// On Windows it is not enough to just recursively remove the contents of a
/// directory and then the directory itself. Deleting does not happen
/// instantaneously, but is delayed by IO being completed in the fs stack and
/// then the last copy of the directory handle being closed.
///
/// Further, typical Windows machines can handle many more concurrent IOs than a
/// single threaded application is capable of submitting: the overlapped (async)
/// calls available do not cover the operations needed to perform directory
/// removal efficiently.
///
/// The `parallel` feature enables the use of a work stealing scheduler to
/// mitigate this limitation: that permits submitting deletions concurrently with
/// directory scanning, and delete sibling directories in parallel. This allows
/// the slight latency of STATUS_DELETE_PENDING to only have logarithmic effect:
/// a very deep tree will pay wall clock time for that overhead per level as the
/// tree traverse completes, but not linearly for every interior not as a simple
/// recursive deletion would result in.
///
/// Earlier versions of this crate moved the contents of the directory being
/// deleted to become siblings of `base_dir`, which required write access to the
/// parent directory under all circumstances; this is no longer done, even when
/// the parallel feature is disabled
/// - though we could re-instate if in-use files turn out to be handled very
///   poorly with this new threaded implementation, or if many people find
///   threads more concerning that moving files outside of their directory
///   structure as part of deletion!
/// - As a result in-use file deletion now leaves files in-situ and blocks the
///   removal, when previously it would leave the file with a nonsense name
///   outside the dir - but not block the removal. Generally speaking
///   applications can choose when to close files, and they should arrange to do
///   so before deleting the directory.
///
/// There is a single small race condition where external side effects may be
/// left: when deleting a hard linked readonly file, the syscalls required are:
/// - open
/// - set rw
/// - unlink (SetFileDispositionDelete)
/// - set ro
///
/// A crash or power failure could lead to the loss of the readonly bit on the
/// hardlinked inode.
///
/// To handle files with names like `CON` and `morse .. .`,  and when a directory
/// structure is so deep it needs long path names the path is first converted to
/// the Win32 file namespace by calling `canonicalize()`.
pub fn remove_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = _remove_dir_contents(path)?;
    let metadata = path.metadata()?;
    if metadata.permissions().readonly() {
        delete_readonly(metadata, &path)?;
    } else {
        log::trace!("removing {}", &path.display());
        fs::remove_dir(&path).map_err(|e| {
            log::debug!("error removing {}", &path.display());
            e
        })?;
        log::trace!("removed {}", &path.display());
    }
    Ok(())
}

/// Returns the canonicalised path, for one of caller's convenience.
pub fn _remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<PathBuf> {
    let path = path.as_ref().canonicalize()?;
    _delete_dir_contents(&path)?;
    Ok(path)
}

fn _delete_dir_contents(path: &PathBuf) -> io::Result<()> {
    log::trace!("scanning {}", &path.display());
    cfg_if::cfg_if! {
        if #[cfg(feature = "parallel")] {
            let iter = path.read_dir()?.par_bridge();
        } else {
            let mut iter = path.read_dir()?;
        }
    }
    iter.try_for_each(|dir_entry| -> io::Result<()> {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        let is_dir = dir_entry.file_type()?.is_dir();
        let dir_path = dir_entry.path();
        if is_dir {
            _delete_dir_contents(&dir_path)?;
        }
        log::trace!("removing {}", &dir_path.display());
        if metadata.permissions().readonly() {
            delete_readonly(metadata, &dir_path).map_err(|e| {
                log::debug!("error removing {}", &dir_path.display());
                e
            })?;
        } else if is_dir {
            fs::remove_dir(&dir_path).map_err(|e| {
                log::debug!("error removing {}", &dir_path.display());
                e
            })?;
        } else {
            fs::remove_file(&dir_path).map_err(|e| {
                log::debug!("error removing {}", &dir_path.display());
                e
            })?;
        }
        log::trace!("removed {}", &dir_path.display());
        Ok(())
    })?;
    log::trace!("scanned {}", &path.display());
    Ok(())
}

// Delete a file or directory that is readonly
fn delete_readonly(metadata: fs::Metadata, path: &Path) -> io::Result<()> {
    // Open, drop the readonly bit, set delete-on-close, close.
    let mut opts = OpenOptions::new();
    opts.access_mode(DELETE | FILE_READ_ATTRIBUTES | FILE_WRITE_ATTRIBUTES);
    opts.custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT);

    let file = opts.open(path)?;
    let mut perms = metadata.permissions();
    perms.set_readonly(false);
    file.set_permissions(perms)?;

    let mut info = FILE_DISPOSITION_INFO {
        DeleteFile: true as u8,
    };
    let result = unsafe {
        SetFileInformationByHandle(
            file.as_raw_handle() as HANDLE,
            FileDispositionInfo,
            &mut info as *mut FILE_DISPOSITION_INFO as *mut c_void,
            size_of::<FILE_DISPOSITION_INFO>() as u32,
        )
    };

    if result == 0 {
        return Err(io::Error::last_os_error());
    }

    file.set_permissions(metadata.permissions())?;
    drop(file);
    Ok(())
}
