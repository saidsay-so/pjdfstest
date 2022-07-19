use nix::{
    errno::Errno,
    fcntl::{open, OFlag},
    sys::stat::{mknod, Mode, SFlag},
    unistd::{chown, mkdir, mkfifo, truncate, unlink, User},
};

#[cfg(any(
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "macos",
    target_os = "ios"
))]
use nix::{sys::stat::FileFlag, unistd::chflags};

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use crate::{
    runner::context::{FileType, TestContext},
    utils::{chmod, lchmod, lchown, link, rename, symlink},
};

crate::test_case! {eloop}
fn eloop(ctx: &mut TestContext) {
    let p1 = ctx
        .create_named(FileType::Symlink(Some(PathBuf::from("loop2"))), "loop1")
        .unwrap();
    let p2 = ctx
        .create_named(FileType::Symlink(Some(PathBuf::from("loop1"))), "loop2")
        .unwrap();
    let p3 = ctx.create(FileType::Regular).unwrap();

    fn assert_eloop_folder<F, T: Debug>(p1: &Path, p2: &Path, f: F)
    where
        F: Fn(&Path) -> nix::Result<T>,
    {
        assert_eq!(f(p1).unwrap_err(), Errno::ELOOP);
        assert_eq!(f(p2).unwrap_err(), Errno::ELOOP);
    }
    fn assert_eloop_final<F, T: Debug>(p1: &Path, p2: &Path, f: F)
    where
        F: Fn(&Path) -> nix::Result<T>,
    {
        assert_eq!(f(&p1.join("test")).unwrap_err(), Errno::ELOOP);
        assert_eq!(f(&p2.join("test")).unwrap_err(), Errno::ELOOP);
    }
    fn assert_eloop_link<F, T: Debug>(p1: &Path, p2: &Path, p3: &Path, f: F)
    where
        F: Fn(&Path, &Path) -> nix::Result<T>,
    {
        assert_eq!(f(&p1.join("test"), &p3).unwrap_err(), Errno::ELOOP);
        assert_eq!(f(&p2.join("test"), &p3).unwrap_err(), Errno::ELOOP);
        assert_eq!(f(&p3, &p1.join("test")).unwrap_err(), Errno::ELOOP);
        assert_eq!(f(&p3, &p2.join("test")).unwrap_err(), Errno::ELOOP);
    }
    fn assert_eloop_all<F, T: Debug>(p1: &Path, p2: &Path, f: F)
    where
        F: Fn(&Path) -> nix::Result<T>,
    {
        assert_eloop_folder(p1, p2, &f);
        assert_eloop_final(p1, p2, f);
    }

    // TODO: Add rmdir upstream
    // assert_eloop(&p1, &p2, |p| rmdir(p, Mode::empty()));
    assert_eloop_final(&p1, &p2, |p| chflags(p, FileFlag::empty()));
    assert_eloop_all(&p1, &p2, |p| chmod(p, Mode::empty()));
    assert_eloop_final(&p1, &p2, |p| lchmod(p, Mode::empty()));
    assert_eloop_all(&p1, &p2, |p| {
        chown(
            p,
            Some(User::from_name("nobody").unwrap().unwrap().uid),
            None,
        )
    });
    assert_eloop_final(&p1, &p2, |p| {
        lchown(
            p,
            Some(User::from_name("nobody").unwrap().unwrap().uid),
            None,
        )
    });
    assert_eloop_link(&p1, &p2, &p3, |p1, p2| link(p1, p2));
    assert_eloop_final(&p1, &p2, |p| mkdir(p, Mode::empty()));
    assert_eloop_final(&p1, &p2, |p| mkfifo(p, Mode::empty()));
    assert_eloop_final(&p1, &p2, |p| mknod(p, SFlag::S_IFCHR, Mode::empty(), 0));
    assert_eloop_final(&p1, &p2, |p| open(p, OFlag::empty(), Mode::empty()));
    assert_eloop_link(&p1, &p2, &p3, |p1, p2| rename(p1, p2));
    assert_eloop_final(&p1, &p2, |p| symlink(Path::new("test"), p));
    assert_eloop_final(&p1, &p2, |p| truncate(p, 0));
    assert_eloop_final(&p1, &p2, |p| unlink(p));
}

crate::test_case! {enotdir => [Regular, Fifo, Block, Char, Socket]}
fn enotdir(ctx: &mut TestContext, ft: FileType) {
    let base_path = ctx.create(ft).unwrap();
    let path = base_path.join("previous_not_dir");
    let dir = ctx.create(FileType::Dir).unwrap();

    fn assert_enotdir<T: Debug, F>(path: &Path, f: F)
    where
        F: Fn(&Path) -> nix::Result<T>,
    {
        assert_eq!(f(path).unwrap_err(), Errno::ENOTDIR);
    }

    // TODO: Add rmdir upstream
    // assert_enotdir(&path, |p| rmdir(p, Mode::empty()));
    assert_enotdir(&path, |p| chflags(p, FileFlag::empty()));
    assert_enotdir(&path, |p| chmod(p, Mode::empty()));
    assert_enotdir(&path, |p| lchmod(p, Mode::empty()));
    assert_enotdir(&path, |p| {
        chown(
            p,
            Some(User::from_name("nobody").unwrap().unwrap().uid),
            None,
        )
    });
    assert_enotdir(&path, |p| {
        lchown(
            p,
            Some(User::from_name("nobody").unwrap().unwrap().uid),
            None,
        )
    });
    assert_enotdir(&path, |p| link(p, &base_path));
    assert_enotdir(&path, |p| link(&*base_path, p));
    assert_enotdir(&path, |p| mkdir(p, Mode::empty()));
    assert_enotdir(&path, |p| mkfifo(p, Mode::empty()));
    assert_enotdir(&path, |p| mknod(p, SFlag::S_IFCHR, Mode::empty(), 0));
    assert_enotdir(&path, |p| open(p, OFlag::O_RDONLY, Mode::empty()));
    assert_enotdir(&path, |p| {
        open(p, OFlag::O_CREAT, Mode::from_bits_truncate(0o644))
    });
    assert_enotdir(&path, |p| rename(&dir, &base_path));
    assert_enotdir(&path, |p| rename(p, Path::new("test")));
    assert_enotdir(&path, |p| rename(&*base_path, p));
    assert_enotdir(&path, |p| symlink(Path::new("test"), p));
    assert_enotdir(&path, |p| truncate(p, 0));
    assert_enotdir(&path, |p| unlink(p));
}

crate::test_case! {enametoolong_comp_max}
fn enametoolong_comp_max(ctx: &mut TestContext) {
    let path = ctx.create_name_max(FileType::Regular).unwrap();

    fn assert_enametoolong<F, T: Debug + std::cmp::PartialEq>(
        invalid_path: &Path,
        valid_path: &Path,
        expected: T,
        f: F,
    ) where
        F: Fn(&Path) -> nix::Result<T>,
    {
        let res = f(invalid_path);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
        assert_eq!(f(invalid_path).unwrap_err(), Errno::ENOTDIR);
    }
}
