use std::fs::symlink_metadata;

use nix::{
    errno::Errno,
    sys::stat::{lstat, stat},
};

use crate::{
    runner::context::{FileType, SerializedTestContext, TestContext},
    test::FileSystemFeature,
    tests::{assert_symlink_ctime_unchanged, AsTimeInvariant, MetadataExt},
    utils::{link, rename},
};

use super::assert_ctime_changed;

crate::test_case! {
    /// rename preserve file metadata
    // rename/00.t
    preserve_metadata => [Regular, Fifo, Block, Char, Socket]
}
fn preserve_metadata(ctx: &mut TestContext, ft: FileType) {
    let old_path = ctx.create(ft).unwrap();
    let new_path = ctx.base_path().join("new");

    let old_path_stat = lstat(&old_path).unwrap();

    assert!(rename(&old_path, &new_path).is_ok());
    assert_eq!(lstat(&old_path).unwrap_err(), Errno::ENOENT);

    let new_path_stat = lstat(&new_path).unwrap();
    assert_eq!(
        old_path_stat.as_time_invariant(),
        new_path_stat.as_time_invariant()
    );

    let link_path = ctx.base_path().join("link");
    link(&new_path, &link_path).unwrap();

    let link_stat = lstat(&link_path).unwrap();
    let new_path_stat = lstat(&new_path).unwrap();
    assert_eq!(
        new_path_stat.as_time_invariant(),
        link_stat.as_time_invariant()
    );
    assert_eq!(link_stat.st_nlink, 2);

    let another_path = ctx.base_path().join("another");
    assert!(rename(&new_path, &another_path).is_ok());
    assert_eq!(lstat(&new_path).unwrap_err(), Errno::ENOENT);

    let another_path_stat = lstat(&another_path).unwrap();
    assert_eq!(
        link_stat.as_time_invariant(),
        another_path_stat.as_time_invariant()
    );
}

crate::test_case! {
    /// rename preserve directory metadata
    // rename/00.t
    preserve_metadata_dir
}
fn preserve_metadata_dir(ctx: &mut TestContext) {
    let old_path = ctx.create(FileType::Dir).unwrap();
    let new_path = ctx.base_path().join("new");

    let old_path_stat = lstat(&old_path).unwrap();

    assert!(rename(&old_path, &new_path).is_ok());
    assert_eq!(lstat(&old_path).unwrap_err(), Errno::ENOENT);

    let new_path_stat = lstat(&new_path).unwrap();
    assert_eq!(
        old_path_stat.as_time_invariant(),
        new_path_stat.as_time_invariant()
    );
}

crate::test_case! {
    /// rename preserve symlink metadata
    // rename/00.t
    preserve_metadata_symlink
}
fn preserve_metadata_symlink(ctx: &mut TestContext) {
    let target = ctx.create(FileType::Regular).unwrap();
    let target_stat = lstat(&target).unwrap();

    let symlink_old_path = ctx.create(FileType::Symlink(Some(target))).unwrap();
    let symlink_stat = lstat(&symlink_old_path).unwrap();
    let sym_target_stat = stat(&symlink_old_path).unwrap();

    assert_ne!(
        symlink_stat.as_time_invariant(),
        sym_target_stat.as_time_invariant()
    );
    assert_eq!(
        sym_target_stat.as_time_invariant(),
        target_stat.as_time_invariant()
    );

    let sym_new_path = ctx.base_path().join("sym_new_path");
    rename(&symlink_old_path, &sym_new_path).unwrap();

    let sym_target_stat = stat(&sym_new_path).unwrap();
    assert_eq!(target_stat, sym_target_stat);
    assert_eq!(lstat(&symlink_old_path).unwrap_err(), Errno::ENOENT);

    let sym_new_stat = lstat(&sym_new_path).unwrap();
    assert_eq!(
        symlink_stat.as_time_invariant(),
        sym_new_stat.as_time_invariant()
    );
}

crate::test_case! {
    /// rename should not update ctime if it fails
    // rename/00.t
    unchanged_ctime_failed, serialized, root => [Regular, Dir, Fifo, Block, Char, Socket, Symlink(None)]
}
fn unchanged_ctime_failed(ctx: &mut SerializedTestContext, ft: FileType) {
    let file = ctx.new_file(ft).mode(0o600).create().unwrap();
    let other_path = ctx.gen_path();
    let user = ctx.get_new_user();
    ctx.as_user(&user, None, || {
        assert_symlink_ctime_unchanged(ctx, &file, || {
            assert!(rename(&file, &other_path).is_err());
        })
    });
}

crate::test_case! {
    /// write access to subdirectory is required to move it to another directory
    // rename/21.t
    write_access_required_subdir, serialized, root
}
fn write_access_required_subdir(ctx: &mut SerializedTestContext) {
    let dir = ctx.new_file(FileType::Dir).mode(0o777).create().unwrap();
    let subdir = ctx
        .new_file(FileType::Dir)
        .name(dir.join("subdir"))
        .mode(0o700)
        .create()
        .unwrap();
    let another_subdir_path = dir.join("another_subdir_path");

    let new_dir = ctx.new_file(FileType::Dir).mode(0o777).create().unwrap();
    let new_dir_subpath = new_dir.join("subpath");

    let user = ctx.get_new_user();
    ctx.as_user(&user, None, || {
        // Check that write permission on containing directory is enough
        // to rename subdirectory. If we rename directory write access
        // to this directory may also be required.
        assert!(matches!(
            rename(&subdir, &another_subdir_path),
            Ok(_) | Err(Errno::EACCES)
        ));

        assert!(matches!(
            rename(&another_subdir_path, &subdir),
            Ok(_) | Err(Errno::EACCES)
        ));

        //TODO: Is it really useful?
        // Check that write permission on containing directory is not enough
        // to move subdirectory from that directory.
        // Actually POSIX says that write access to `dir` and `new_dir` may be enough
        // to move `subdir`.
        assert!(matches!(
            rename(&subdir, &new_dir_subpath),
            Ok(_) | Err(Errno::EACCES)
        ));
    });

    // Check that write permission on containing directory (${n2}) is enough
    // to move file (${n0}) from that directory.
    let file = ctx
        .new_file(FileType::Regular)
        .name(dir.join("file"))
        .mode(0o600)
        .create()
        .unwrap();

    ctx.as_user(&user, None, || {
        let new_path = new_dir.join("file");
        assert!(rename(&file, &new_path).is_ok());
    })
}

crate::test_case! {
    /// rename should update ctime if it succeeds
    // rename/22.t
    changed_ctime_success, FileSystemFeature::RenameCtime => [Regular, Dir, Fifo, Block, Char, Socket, Symlink(None)]
}
fn changed_ctime_success(ctx: &mut TestContext, ft: FileType) {
    let old_path = ctx.create(ft).unwrap();
    let new_path = ctx.base_path().join("new");

    let old_path_ctime = symlink_metadata(&old_path).unwrap().ctime_ts();

    ctx.nap();

    assert!(rename(&old_path, &new_path).is_ok());

    let new_path_ctime = symlink_metadata(&new_path).unwrap().ctime_ts();

    assert!(new_path_ctime > old_path_ctime);
}

crate::test_case! {
    /// rename succeeds when to is multiply linked
    // rename/23.t
    to_multiply_linked => [Regular, Fifo, Block, Char, Socket]
}
fn to_multiply_linked(ctx: &mut TestContext, ft: FileType) {
    let src = ctx.create(ft.clone()).unwrap();
    let dst = ctx.create(ft).unwrap();

    let dst_link = ctx.base_path().join("dst_link");
    link(&dst, &dst_link).unwrap();
    let dst_link_stat = lstat(&dst_link).unwrap();
    assert_eq!(dst_link_stat.st_nlink, 2);

    assert_ctime_changed(ctx, &dst_link, || {
        assert!(rename(&src, &dst).is_ok());
    });

    let dst_link_stat = lstat(&dst_link).unwrap();
    assert_eq!(dst_link_stat.st_nlink, 1);
}

crate::test_case! {
    /// rename of a directory updates its .. link
    // rename/24.t
    updates_link_parent
}
fn updates_link_parent(ctx: &mut TestContext) {
    let src_parent = ctx.create(FileType::Dir).unwrap();
    let dst_parent = ctx.create(FileType::Dir).unwrap();
    let dst = dst_parent.join("dst");
    let src = ctx
        .new_file(FileType::Dir)
        .name(src_parent.join("src"))
        .create()
        .unwrap();

    // Initial conditions
    let src_parent_stat = lstat(&src_parent).unwrap();
    let dst_parent_stat = lstat(&dst_parent).unwrap();

    assert_eq!(src_parent_stat.st_nlink, 3);
    assert_eq!(dst_parent_stat.st_nlink, 2);
    let dotdot_stat = lstat(&src.join("..")).unwrap();
    assert_eq!(src_parent_stat.st_ino, dotdot_stat.st_ino);

    assert!(rename(&src, &dst).is_ok());

    let src_parent_stat = lstat(&src_parent).unwrap();
    let dst_parent_stat = lstat(&dst_parent).unwrap();
    assert_eq!(src_parent_stat.st_nlink, 2);
    assert_eq!(dst_parent_stat.st_nlink, 3);
    let dotdot_stat = lstat(&dst.join("..")).unwrap();
    assert_eq!(dst_parent_stat.st_ino, dotdot_stat.st_ino);
}