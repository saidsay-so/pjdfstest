use std::fs::FileType;

use nix::unistd::mkdir;

use crate::runner::context::{SerializedTestContext, TestContext};

use super::mksyscalls::{
    changed_time_fields_success_builder, permission_bits_from_mode_builder,
    uid_gid_eq_euid_or_parent_uid_egid_builder,
};

crate::test_case! {
    /// POSIX: The file permission bits of the new directory shall be initialized from
    /// mode. These file permission bits of the mode argument shall be modified by the
    /// process' file creation mask.
    permission_bits_from_mode, serialized
}
fn permission_bits_from_mode(ctx: &mut SerializedTestContext) {
    permission_bits_from_mode_builder(ctx, mkdir, FileType::is_dir);
}

crate::test_case! {
    /// POSIX: The directory's user ID shall be set to the process' effective user ID.
    /// The directory's group ID shall be set to the group ID of the parent directory
    /// or to the effective group ID of the process.
    uid_gid_eq_euid_egid, serialized, root
}
fn uid_gid_eq_euid_egid(ctx: &mut SerializedTestContext) {
    uid_gid_eq_euid_or_parent_uid_egid_builder(ctx, mkdir);
}

crate::test_case! {
    /// POSIX: Upon successful completion, mkdir() shall mark for update the st_atime,
    /// st_ctime, and st_mtime fields of the directory. Also, the st_ctime and
    /// st_mtime fields of the directory that contains the new entry shall be marked
    /// for update.
    changed_time_fields_success
}
fn changed_time_fields_success(ctx: &mut TestContext) {
    changed_time_fields_success_builder(ctx, mkdir);
}
