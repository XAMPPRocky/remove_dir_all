use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::prelude::FromRawFd;
use std::path::Path;
use std::{fs, os::unix::prelude::AsRawFd};

use cvt::cvt;
use libc::{self, fcntl, F_DUPFD_CLOEXEC};

use super::io::Io;

pub(crate) struct UnixIo;

impl Io for UnixIo {
    type UniqueIdentifier = ();

    fn duplicate_fd(f: &mut fs::File) -> io::Result<fs::File> {
        let source_fd = f.as_raw_fd();
        // F_DUPFD_CLOEXEC seems to be quite portable, but we should be prepared
        // to add in more codepaths here.
        let fd = cvt(unsafe { fcntl(source_fd, F_DUPFD_CLOEXEC, 0) })?;
        Ok(unsafe { File::from_raw_fd(fd) })
    }

    fn open_dir(p: &Path) -> io::Result<fs::File> {
        let mut options = OpenOptions::new();
        options.read(true);
        options.custom_flags(libc::O_NOFOLLOW);
        options.open(p)
    }

    fn unique_identifier(_d: &fs::File) -> io::Result<Self::UniqueIdentifier> {
        todo!()
    }

    fn is_eloop(e: &io::Error) -> bool {
        e.raw_os_error() == Some(libc::ELOOP)
    }
}
