use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    io::{self, Result},
    mem::{size_of, MaybeUninit},
    os::windows::fs::OpenOptionsExt,
    os::windows::prelude::AsRawHandle,
    os::windows::prelude::*,
    path::Path,
};

use windows_sys::Win32::{
    Foundation::{DuplicateHandle, DUPLICATE_SAME_ACCESS, ERROR_CANT_RESOLVE_FILENAME, HANDLE},
    Storage::FileSystem::{
        FileIdInfo, GetFileInformationByHandleEx, FILE_FLAG_BACKUP_SEMANTICS,
        FILE_FLAG_OPEN_REPARSE_POINT, FILE_ID_INFO,
    },
    System::Threading::GetCurrentProcess,
};

use super::io::Io;

pub(crate) struct WindowsIo;

// basically FILE_ID_INFO but declared primitives to permit derives.
#[derive(Debug, PartialEq)]
pub(crate) struct VSNFileId {
    vsn: u64,
    file_id: [u8; 16],
}

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

    type UniqueIdentifier = VSNFileId;

    fn unique_identifier(d: &File) -> io::Result<Self::UniqueIdentifier> {
        let mut info = MaybeUninit::<FILE_ID_INFO>::uninit();
        let bool_result = unsafe {
            GetFileInformationByHandleEx(
                d.as_raw_handle() as HANDLE,
                FileIdInfo,
                info.as_mut_ptr() as *mut c_void,
                size_of::<FILE_ID_INFO>() as u32,
            )
        };
        if bool_result == 0 {
            return Err(io::Error::last_os_error());
        }
        let info = unsafe { info.assume_init() };
        Ok(VSNFileId {
            vsn: info.VolumeSerialNumber,
            file_id: info.FileId.Identifier,
        })
    }

    fn is_eloop(e: &io::Error) -> bool {
        e.raw_os_error() == Some(ERROR_CANT_RESOLVE_FILENAME as i32)
    }
}
