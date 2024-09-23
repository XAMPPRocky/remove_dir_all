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
        // When the `NO_FOLLOW` flag is set, POSIX specifies that ELOOP be turned
        // if the trailing component is a symlink.
        // However, not all platforms follow POSIX on this.
        //
        // FreeBSD uses EMLINK: https://man.freebsd.org/cgi/man.cgi?query=open&sektion=2&n=1#end
        // NetBSD uses EFTYPE: https://man.netbsd.org/open.2#ERRORS
        cfg_if::cfg_if! {
            if #[cfg(target_os = "freebsd")] {
                matches!(e.raw_os_error(), Some(libc::ELOOP) | Some(libc::EMLINK))
            } else if #[cfg(target_os = "netbsd")] {
                matches!(e.raw_os_error(), Some(libc::ELOOP) | Some(libc::EFTYPE))
            } else {
                e.raw_os_error() == Some(libc::ELOOP)
            }
        }
    }
}
