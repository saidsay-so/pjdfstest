use std::{fs::metadata, os::unix::fs::MetadataExt};

use crate::{
    context::{FileType, SerializedTestContext},
    test::TestContext,
    tests::{assert_ctime_changed, assert_ctime_unchanged},
    utils::{chmod, ALLPERMS},
};

use nix::{
    errno::Errno,
    libc::mode_t,
    sys::stat::{lstat, stat, Mode},
    unistd::{chown, Uid, User},
};

use super::errors::{
    efault::efault_path_test_case,
    eloop::{eloop_comp_test_case, eloop_final_comp_test_case},
    enametoolong::{enametoolong_comp_test_case, enametoolong_path_test_case},
    enoent::{
        enoent_comp_test_case, enoent_named_file_test_case, enoent_symlink_named_file_test_case,
    },
    enotdir::enotdir_comp_test_case,
    erofs::erofs_named_test_case,
};

#[cfg(lchmod)]
mod lchmod;

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
const ALLPERMS_STICKY: nix::libc::mode_t = ALLPERMS | Mode::S_ISVTX.bits();

// chmod/00.t:L24
crate::test_case! {
    /// chmod successfully change permissions
    change_perm => [Regular, Dir, Fifo, Block, Char, Socket]
}
fn change_perm(ctx: &mut TestContext, f_type: FileType) {
    let path = ctx.create(f_type).unwrap();
    let expected_mode = Mode::from_bits_truncate(0o111);

    assert!(chmod(&path, expected_mode).is_ok());

    let actual_mode = stat(&path).unwrap().st_mode;

    assert_eq!(actual_mode & ALLPERMS, expected_mode.bits());

    // We test if it applies through symlinks
    let symlink_path = ctx.create(FileType::Symlink(Some(path.clone()))).unwrap();
    let link_mode = lstat(&symlink_path).unwrap().st_mode;
    let expected_mode = Mode::from_bits_truncate(0o222);

    assert!(chmod(&symlink_path, expected_mode).is_ok());

    let actual_mode = stat(&path).unwrap().st_mode;
    let actual_sym_mode = stat(&symlink_path).unwrap().st_mode;
    assert_eq!(actual_mode & ALLPERMS, expected_mode.bits());
    assert_eq!(actual_sym_mode & ALLPERMS, expected_mode.bits());

    let actual_link_mode = lstat(&symlink_path).unwrap().st_mode;
    assert_eq!(link_mode & ALLPERMS, actual_link_mode & ALLPERMS);
}

// chmod/00.t:L58
crate::test_case! {
    /// chmod updates ctime when it succeeds
    update_ctime => [Regular, Dir, Fifo, Block, Char, Socket]
}
fn update_ctime(ctx: &mut TestContext, f_type: FileType) {
    let path = ctx.create(f_type).unwrap();
    assert_ctime_changed(ctx, &path, || {
        assert!(chmod(&path, Mode::from_bits_truncate(0o111)).is_ok());
    });
}

// chmod/00.t:L89
crate::test_case! {
    /// chmod does not update ctime when it fails
    failed_chmod_unchanged_ctime, serialized, root => [Regular, Dir, Fifo, Block, Char, Socket]
}
fn failed_chmod_unchanged_ctime(ctx: &mut SerializedTestContext, f_type: FileType) {
    let path = ctx.create(f_type).unwrap();
    let user = ctx.get_new_user();
    assert_ctime_unchanged(ctx, &path, || {
        ctx.as_user(user, None, || {
            assert!(chmod(&path, Mode::from_bits_truncate(0o111)).is_err());
        });
    });
}

crate::test_case! {
    /// S_ISGID bit shall be cleared upon successful return from chmod of a regular file
    /// if the calling process does not have appropriate privileges, and if
    /// the group ID of the file does not match the effective group ID or one of the
    /// supplementary group IDs
    clear_isgid_bit, serialized, root
}
fn clear_isgid_bit(ctx: &mut SerializedTestContext) {
    let path = ctx.create(FileType::Regular).unwrap();
    assert!(chmod(&path, Mode::from_bits_truncate(0o0755)).is_ok());

    let user = ctx.get_new_user();

    chown(&path, Some(user.uid), Some(user.gid)).unwrap();

    let expected_mode = Mode::from_bits_truncate(0o2755);
    ctx.as_user(user, None, || {
        chmod(&path, expected_mode).unwrap();
    });

    let actual_mode = stat(&path).unwrap().st_mode;
    assert_eq!(actual_mode & 0o7777, expected_mode.bits());

    let expected_mode = Mode::from_bits_truncate(0o0755);
    ctx.as_user(user, None, || {
        assert!(chmod(&path, expected_mode).is_ok());
    });

    let actual_mode = stat(&path).unwrap().st_mode;
    assert_eq!(actual_mode & 0o7777, expected_mode.bits());
    //TODO: FreeBSD "S_ISGID should be removed and chmod(2) should success and FreeBSD returns EPERM."
}

// chmod/01.t
enotdir_comp_test_case!(chmod(~path, Mode::empty()));

// chmod/02.t
enametoolong_comp_test_case!(chmod(~path, Mode::empty()));

// chmod/03.t
enametoolong_path_test_case!(chmod(~path, Mode::empty()));

// chmod/04.t
enoent_named_file_test_case!(chmod(~path, Mode::empty()));
enoent_comp_test_case!(chmod(~path, Mode::empty()));
enoent_symlink_named_file_test_case!(chmod(~path, Mode::empty()));

// chmod/06.t
eloop_comp_test_case!(chmod(~path, Mode::empty()));

// chmod/06.t
eloop_final_comp_test_case!(chmod(~path, Mode::empty()));

crate::test_case! {
    /// chmod returns EPERM if the operation would change the ownership, but the effective user ID is not the super-user
    // chmod/07.t
    chmod_not_owner, serialized, root
}
fn chmod_not_owner(ctx: &mut SerializedTestContext) {
    let user = ctx.get_new_user();
    chown(ctx.base_path(), Some(user.uid), Some(user.gid)).unwrap();

    let file = ctx.create(FileType::Regular).unwrap();
    chown(&file, Some(user.uid), Some(user.gid)).unwrap();

    let mode = Mode::from_bits_truncate(0o642);
    let new_mode = Mode::from_bits_truncate(0o641);

    ctx.as_user(user, None, || {
        assert!(chmod(&file, mode).is_ok());
        let file_stat = metadata(&file).unwrap();
        assert_eq!(file_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });

    let other_user = ctx.get_new_user();
    ctx.as_user(other_user, None, || {
        assert_eq!(chmod(&file, new_mode), Err(Errno::EPERM));
        let file_stat = metadata(&file).unwrap();
        assert_eq!(file_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });

    let current = User::from_uid(Uid::effective()).unwrap().unwrap();
    chown(&file, Some(current.uid), Some(current.gid)).unwrap();

    ctx.as_user(user, None, || {
        assert_eq!(chmod(&file, new_mode), Err(Errno::EPERM));
        let file_stat = metadata(&file).unwrap();
        assert_eq!(file_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });

    // symlink
    let link = ctx.create(FileType::Symlink(Some(file.clone()))).unwrap();
    chown(&link, Some(user.uid), Some(user.gid)).unwrap();
    chown(&file, Some(user.uid), Some(user.gid)).unwrap();

    ctx.as_user(user, None, || {
        assert!(chmod(&link, mode).is_ok());
        let link_stat = metadata(&link).unwrap();
        assert_eq!(link_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });

    let other_user = ctx.get_new_user();
    ctx.as_user(other_user, None, || {
        assert_eq!(chmod(&link, new_mode), Err(Errno::EPERM));
        let link_stat = metadata(&link).unwrap();
        assert_eq!(link_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });

    chown(&link, Some(current.uid), Some(current.gid)).unwrap();

    ctx.as_user(user, None, || {
        assert_eq!(chmod(&link, new_mode), Err(Errno::EPERM));
        let link_stat = metadata(&link).unwrap();
        assert_eq!(link_stat.mode() as mode_t & ALLPERMS, mode.bits());
    });
}

mod flag {
    use super::*;
    use crate::tests::errors::eperm::flag::immutable_append_named_test_case;

    const EXPECTED_MODE: Mode = Mode::from_bits_truncate(0o100);
    // chmod/08.t
    immutable_append_named_test_case!(chmod, |path| chmod(path, EXPECTED_MODE), |path| metadata(
        path
    )
    .map_or(false, |m| m.mode() as mode_t & ALLPERMS
        == EXPECTED_MODE.bits()));
}

// chmod/09.t
erofs_named_test_case!(chmod(~path, Mode::empty()));

// chmod/10.t
efault_path_test_case!(chmod, |ptr| nix::libc::chmod(ptr, 0));

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
crate::test_case! {
    /// chmod returns EFTYPE if the effective user ID is not the super-user,
    /// the mode includes the sticky bit (S_ISVTX),
    /// and path does not refer to a directory
    // chmod/11.t
    eftype, serialized, root => [Regular, Fifo, Block, Char, Socket]
}
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
fn eftype(ctx: &mut SerializedTestContext, ft: FileType) {
    use crate::utils::lchmod;

    let user = ctx.get_new_user();

    let original_mode = Mode::from_bits_truncate(0o640);
    let file = ctx
        .new_file(ft)
        .mode(original_mode.bits())
        .create()
        .unwrap();
    chown(&file, Some(user.uid), Some(user.gid)).unwrap();
    let new_mode = Mode::from_bits_truncate(0o644);
    let link = ctx.create(FileType::Symlink(Some(file.clone()))).unwrap();

    ctx.as_user(user, None, || {
        assert_eq!(chmod(&file, new_mode | Mode::S_ISVTX), Err(Errno::EFTYPE));
    });
    let file_stat = stat(&file).unwrap();
    assert_eq!(file_stat.st_mode & ALLPERMS_STICKY, original_mode.bits());

    ctx.as_user(user, None, || {
        assert_eq!(
            chmod(&link, original_mode | Mode::S_ISVTX),
            Err(Errno::EFTYPE)
        );
    });
    let file_stat = stat(&link).unwrap();
    assert_eq!(file_stat.st_mode & ALLPERMS_STICKY, original_mode.bits());

    // lchmod

    let mode = Mode::from_bits_truncate(0o621) | Mode::S_ISVTX;
    ctx.as_user(user, None, || {
        assert_eq!(lchmod(&file, mode), Err(Errno::EFTYPE));
    });

    let file_stat = lstat(&file).unwrap();
    assert_eq!(file_stat.st_mode & ALLPERMS_STICKY, original_mode.bits());
}
