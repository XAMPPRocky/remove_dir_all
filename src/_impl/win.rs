use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    io::{self, Result},
    mem::MaybeUninit,
    os::windows::fs::OpenOptionsExt,
    os::windows::prelude::AsRawHandle,
    os::windows::prelude::*,
    path::Path,
};

use windows_sys::Win32::{
    Foundation::{DuplicateHandle, DUPLICATE_SAME_ACCESS, HANDLE},
    Storage::FileSystem::{FILE_FLAG_BACKUP_SEMANTICS, FILE_FLAG_OPEN_REPARSE_POINT},
    System::Threading::GetCurrentProcess,
};

use super::io::Io;

pub(crate) struct WindowsIo;

impl Io for WindowsIo {
    fn duplicate_fd(f: &mut File) -> io::Result<File> {
        let mut new_handle: MaybeUninit<*mut c_void> = MaybeUninit::uninit();

        let result = unsafe {
            DuplicateHandle(
                GetCurrentProcess(),
                f.as_raw_handle() as HANDLE,
                GetCurrentProcess(),
                new_handle.as_mut_ptr() as *mut HANDLE,
                0,
                false as i32,
                DUPLICATE_SAME_ACCESS,
            )
        };
        if result == 0 {
            return Err(std::io::Error::last_os_error());
        }

        let new_handle = unsafe { new_handle.assume_init() };
        Ok(unsafe { File::from_raw_handle(new_handle) })
    }

    fn open_dir(p: &Path) -> Result<File> {
        let mut options = OpenOptions::new();
        options.read(true);
        options.custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT);
        let maybe_dir = options.open(p)?;
        if maybe_dir.metadata()?.is_symlink() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Path is a directory link, not directory",
            ));
        }
        Ok(maybe_dir)
    }
}
