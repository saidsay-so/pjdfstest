use crate::{
    runner::context::{FileType, SerializedTestContext},
    utils::{lchown, link},
};

#[cfg(not(target_os = "linux"))]
use {crate::runner::context::TestContext, nix::unistd::unlink};

use nix::{errno::Errno, unistd::chown};

#[cfg(any(
    target_os = "openbsd",
    target_os = "netbsd",
    target_os = "freebsd",
    target_os = "dragonfly",
    target_os = "macos",
    target_os = "ios",
    target_os = "watchos",
))]
mod flag;

// From https://pubs.opengroup.org/onlinepubs/9699919799/functions/unlink.html
//
// The standard developers reviewed TR 24715-2006 and noted that LSB-conforming implementations
// may return [EISDIR] instead of [EPERM] when unlinking a directory.
// A change to permit this behavior by changing the requirement for [EPERM] to [EPERM] or [EISDIR] was considered,
// but decided against since it would break existing strictly conforming and conforming applications.
// Applications written for portability to both POSIX.1-2017 and the LSB should be prepared to handle either error code.
#[cfg(not(target_os = "linux"))]
crate::test_case! {
    /// unlink may return EPERM if the named file is a directory
    // unlink/08.t
    unlink_dir
}
#[cfg(not(target_os = "linux"))]
fn unlink_dir(ctx: &mut TestContext) {
    let dir = ctx.create(FileType::Dir).unwrap();
    assert!(matches!(unlink(&dir), Ok(_) | Err(Errno::EPERM)));
}

// #[cfg(target_os = "linux")]
// crate::test_case! {
//     /// unlink return EISDIR if the named file is a directory
//     // unlink/08.t
//     unlink_dir
// }
// #[cfg(target_os = "linux")]
// fn unlink_dir(ctx: &mut TestContext) {
//     let dir = ctx.create(FileType::Dir).unwrap();
//     assert!(matches!(unlink(&dir), Err(Errno::EISDIR)));
// }

crate::test_case! {
    /// link returns EPERM if the source file is a directory
    // link/11.t
    link_source_dir, serialized, root
}
fn link_source_dir(ctx: &mut SerializedTestContext) {
    let src = ctx.create(FileType::Dir).unwrap();
    // TODO: Doesn't seem to be an error for SunOS with UFS?
    assert_eq!(link(&src, &ctx.gen_path()), Err(Errno::EPERM));

    let user = ctx.get_new_user();
    chown(ctx.base_path(), Some(user.uid), Some(user.gid)).unwrap();
    chown(&src, Some(user.uid), Some(user.gid)).unwrap();

    ctx.as_user(&user, None, || {
        assert_eq!(link(&src, &ctx.gen_path()), Err(Errno::EPERM));
    })
}

crate::test_case! {
    /// chown returns EPERM if the operation would change the ownership, but the effective user ID is not the super-user and the process is not an owner of the file
    chown_euid_not_root_not_owner, serialized, root => [Regular, Dir, Fifo, Block, Char, Socket]
}
fn chown_euid_not_root_not_owner(ctx: &mut SerializedTestContext, ft: FileType) {
    let user = ctx.get_new_user();
    chown(ctx.base_path(), Some(user.uid), Some(user.gid)).unwrap();

    let file = ctx.create(ft).unwrap();
    chown(&file, Some(user.uid), Some(user.gid)).unwrap();

    let another_user = ctx.get_new_user();

    ctx.as_user(&user, None, || {
        assert_eq!(
            chown(&file, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            chown(&file, Some(user.uid), Some(user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            chown(&file, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&user, None, || {
        assert_eq!(
            chown(&file, None, Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });

    let link = ctx.create(FileType::Symlink(Some(file))).unwrap();

    ctx.as_user(&user, None, || {
        assert_eq!(
            chown(&link, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            chown(&link, Some(user.uid), Some(user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            chown(&link, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&user, None, || {
        assert_eq!(
            chown(&link, None, Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
}

crate::test_case! {
    /// chown returns EPERM if the operation would change the ownership, but the effective user ID is not the super-user and the process is not an owner of the file
    chown_euid_not_root_not_owner_symlink, serialized, root
}
fn chown_euid_not_root_not_owner_symlink(ctx: &mut SerializedTestContext) {
    let user = ctx.get_new_user();
    chown(ctx.base_path(), Some(user.uid), Some(user.gid)).unwrap();

    let file = ctx.create(FileType::Symlink(None)).unwrap();

    let another_user = ctx.get_new_user();

    ctx.as_user(&user, None, || {
        assert_eq!(
            lchown(&file, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            lchown(&file, Some(user.uid), Some(user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&another_user, None, || {
        assert_eq!(
            lchown(&file, Some(another_user.uid), Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
    ctx.as_user(&user, None, || {
        assert_eq!(
            lchown(&file, None, Some(another_user.gid)),
            Err(Errno::EPERM)
        );
    });
}
