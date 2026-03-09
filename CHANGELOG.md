# Unreleased

## Bug fixes

- Unix: `open_dir_at` errors other than symlink-detection (`ELOOP`/`EMLINK`/
  `EFTYPE`) that indicate a non-directory entry — specifically `ENXIO` (returned
  when opening an `AF_UNIX` socket) and `ENOTDIR` (returned when `O_DIRECTORY`
  is in effect) — are now treated as "not a directory" and the entry is removed
  with `unlink_at` rather than propagating an error. Fixes removal of
  directories containing Unix-domain sockets. (#82)

# 1.0.0

## New features

- New `Builder` and `Remover` structs provide a configurable API for
  controlling parallel deletion at runtime, exposed via the `remove_dir_all`
  crate root and the `remove-dir-all` CLI binary. (#80)

## Other changes

- macOS: parallel deletion is now disabled by default due to a global kernel
  lock in APFS that causes thrashing when `readdir` runs concurrently across
  threads. Parallelism can still be enabled explicitly via `Builder`. (#80)

# 0.8.4

## Bug fixes

- Unix: correctly detect symlinks on FreeBSD (`EMLINK`) and NetBSD (`EFTYPE`),
  which do not use `ELOOP` in response to `O_NOFOLLOW`. (#76)

# 0.8.3

## Other changes

- Windows: simplified implementation using `fs_at`'s unified deletion path;
  removed significant internal dead code. (#72)

# 0.8.2

## New features

- `RemoveDir` trait is now public. (#59)
- Added `remove-dir-all` CLI binary (opt-in via the `cli` Cargo feature). (#61)

## Bug fixes

- Windows: use `fs_at`'s POSIX-deletion support to correctly handle in-use
  files on Windows, fixing #34. (#64)

## Other changes

- Improved crate documentation. (#58)

# 0.8.1

## Other changes

- Fix use of fcntl, missing undocumented extra argument.

# 0.8.0

## Security changes

- Fix TOCTOU race conditions both inside the implementation of functions and the
  contract: functions now only operate on directories. Callers wanting to
  process the contents of a symlink (e.g. for remove_dir_contents) should
  resolve the symlink themselves. This is an API break from 0.7.0, but the previous behaviour was insecure.

  This is due to the same code pattern as caused CVE-2022-21658 in Rust itself:
  it was possible to trick a privileged process doing a recursive delete in an
  attacker controlled directory into deleting privileged files, on all operating
  systems.

  For instance, consider deleting a tree called 'etc' in a parent directory
  called 'p'. Between calling `remove_dir_all("a")` and remove_dir_all("a")
  actually starting its work, the attacker can move 'p' to 'p-prime', and
  replace 'p' with a symlink to '/'. Then the privileged process deletes 'p/etc'
  which is actually /etc, and now your system is broken. There are some
  mitigations for this exact scenario, such as CWD relative file lookup, but
  they are not guaranteed - any code using absolute paths will not have that
  protection in place.

  The same attack could be performed at any point in the directory tree being
  deleted: if 'a' contains a child directory called 'etc', attacking the
  deletion by replacing 'a' with a link is possible.

  The new code in this release mitigates the attack within the directory tree
  being deleted by using file-handle relative operations: to open 'a/etc', the
  path 'etc' relative to 'a' is opened, where 'a' is represented by a file
  descriptor (Unix) or handle (Windows). With the exception of the entry points
  into the directory deletion logic, this is robust against manipulation of the
  directory hierarchy, and remove_dir_all will only delete files and directories
  contained in the tree it is deleting.

  The entry path however is a challenge - as described above, there are some
  potential mitigations, but since using them must be done by the calling code,
  it is hard to be confident about the security properties of the path based
  interface.

  The new extension trait `RemoveDir` provides an interface where it is much
  harder to get it wrong.

  `somedir.remove_dir_contents("name-of-child")`.

  Callers can then make their own security evaluation about how to securely get
  a directory handle. That is still not particularly obvious, and we're going to
  follow up with a helper of some sort (probably in the `fs_at` crate). Once
  that is available, the path based entry points will get deprecated.

  In the interim, processes that might run with elevated privileges should
  figure out how to securely identify the directory they are going to delete, to
  avoid the initial race. Pragmatically, other processes should be fine with the
  path based entry points : this is the same interface `std::fs::remove_dir_all`
  offers, and an unprivileged process running in an attacker controlled
  directory can't do anything that the attacker can't already do.

  tl;dr: state shared with threat actors makes things dangerous; library
  functions cannot assume anything about the particular threat model of a
  program and must err on the side of caution.

## Other changes

- Made feature to control use of rayon off-by-default for easier integration by
  other crates.

# 0.7.0

- add remove_dir_contents and ensure_empty_dir

# 0.6.1

- update author
- update README.md

# 0.6.0

- Added threaded deletion on windows
- requires edition 2018 to build

# 0.5.3

- lints and doc fixes

# 0.5.2

- Added support for `aarch64-pc-windows-msvc`.

# 0.5.1

- Fixed deletion of readonly items.

# 0.5.0

- Upgraded to winapi 0.3.
