use std::fs::symlink_metadata;
use std::os::unix::fs::MetadataExt as StdMetadataExt;

use std::{fs::metadata, path::Path};

#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
use nix::sys::stat::stat;
use nix::sys::time::TimeSpec;

use crate::test::TestContext;

pub mod chmod;
pub mod ftruncate;
pub mod link;
pub mod posix_fallocate;
pub mod rename;
pub mod rmdir;
pub mod symlink;
pub mod unlink;
pub mod utimensat;

/// A handy extention to std::os::unix::fs::MetadataExt
trait MetadataExt: StdMetadataExt {
    /// Return the file's last accessed time as a `TimeSpec`, including
    /// fractional component.
    fn atime_ts(&self) -> TimeSpec {
        TimeSpec::new(self.atime(), self.atime_nsec())
    }

    /// Return the file's last changed time as a `TimeSpec`, including
    /// fractional component.
    fn ctime_ts(&self) -> TimeSpec {
        TimeSpec::new(self.ctime(), self.ctime_nsec())
    }

    /// Return the file's last modified time as a `TimeSpec`, including
    /// fractional component.
    fn mtime_ts(&self) -> TimeSpec {
        TimeSpec::new(self.mtime(), self.mtime_nsec())
    }
}

impl<T: StdMetadataExt> MetadataExt for T {}

/// Metadata which isn't related to time.
#[derive(Debug, PartialEq)]
struct InvariantTimeMetadata {
    st_dev: nix::libc::dev_t,
    st_ino: nix::libc::ino_t,
    st_mode: nix::libc::mode_t,
    st_nlink: nix::libc::nlink_t,
    st_uid: nix::libc::uid_t,
    st_gid: nix::libc::gid_t,
    st_rdev: nix::libc::dev_t,
    st_size: nix::libc::off_t,
    st_blksize: nix::libc::blksize_t,
    st_blocks: nix::libc::blkcnt_t,
}

trait AsTimeInvariant {
    fn as_time_invariant(&self) -> InvariantTimeMetadata;
}

impl AsTimeInvariant for nix::sys::stat::FileStat {
    fn as_time_invariant(&self) -> InvariantTimeMetadata {
        InvariantTimeMetadata {
            st_dev: self.st_dev,
            st_ino: self.st_ino,
            st_mode: self.st_mode,
            st_nlink: self.st_nlink,
            st_uid: self.st_uid,
            st_gid: self.st_gid,
            st_rdev: self.st_rdev,
            st_size: self.st_size,
            st_blksize: self.st_blksize,
            st_blocks: self.st_blocks,
        }
    }
}

#[cfg(any(
    target_os = "freebsd",
    target_os = "ios",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd"
))]
// Note: can't be a method of MetadataExt, because StdMetadataExt lacks a
// birthtime() method.
fn birthtime_ts(path: &Path) -> TimeSpec {
    let sb = stat(path).unwrap();
    TimeSpec::new(sb.st_birthtime, sb.st_birthtime_nsec)
}

/// Assert that a certain operation changes the ctime of a file.
fn assert_ctime_changed<F>(ctx: &TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let ctime_before = metadata(&path).unwrap().ctime_ts();

    ctx.nap();

    f();

    let ctime_after = metadata(&path).unwrap().ctime_ts();
    assert!(ctime_after > ctime_before);
}

/// Assert that a certain operation changes the mtime of a file.
fn assert_mtime_changed<F>(ctx: &TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let mtime_before = metadata(&path).unwrap().mtime_ts();

    ctx.nap();

    f();

    let mtime_after = metadata(&path).unwrap().mtime_ts();
    assert!(mtime_after > mtime_before);
}

/// Assert that a certain operation changes the ctime of a file without following symlinks.
fn assert_symlink_ctime_changed<F>(ctx: &mut TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let ctime_before = symlink_metadata(&path).unwrap().ctime_ts();

    ctx.nap();

    f();

    let ctime_after = symlink_metadata(&path).unwrap().ctime_ts();
    assert!(ctime_after > ctime_before);
}

/// Assert that a certain operation does not change the ctime of a file.
fn assert_ctime_unchanged<F>(ctx: &TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let ctime_before = metadata(&path).unwrap().ctime_ts();

    ctx.nap();

    f();

    let ctime_after = metadata(&path).unwrap().ctime_ts();
    assert!(ctime_after == ctime_before);
}

/// Assert that a certain operation does not change the mtime of a file.
fn assert_mtime_unchanged<F>(ctx: &TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let mtime_before = metadata(&path).unwrap().mtime_ts();

    ctx.nap();

    f();

    let mtime_after = metadata(&path).unwrap().mtime_ts();
    assert!(mtime_after == mtime_before);
}

/// Assert that a certain operation does not change the ctime without following symlinks.
fn assert_symlink_ctime_unchanged<F>(ctx: &TestContext, path: &Path, f: F)
where
    F: FnOnce(),
{
    let ctime_before = symlink_metadata(&path).unwrap().ctime_ts();

    ctx.nap();

    f();

    let ctime_after = symlink_metadata(&path).unwrap().ctime_ts();
    assert!(ctime_after == ctime_before);
}
