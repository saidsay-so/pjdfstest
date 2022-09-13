//! Tests for readattr (called ACL_READ_ATTRIBUTES) on FreeBSD 





#[cfg(any(target_os = "macos", target_os = "freebsd"))]
crate::test_case! {
    /// ACL_READ_ATTRIBUTES allows a user to read file attributes
    // granular/01.t
    allowed, serialized, root, FileSystemFeature::Nfsv4Acls
}
#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn allowed(ctx: &mut SerializedTestContext) {
    let path = ctx.new_file(FileType::Regular).mode(0o644).create().unwrap();
    let user = ctx.get_new_user();

    prependacl(&path, &format!("deny::group:{}:readattr", user.gid));

    ctx.as_user(&user, None, || {
        let e = stat(&path).unwrap_err(); // "user" can no longer stat it
        assert_eq!(Errno::EACCES, e);
    });

    prependacl(&path, &format!("allow::user:{}:readattr", user.uid));
    ctx.as_user(&user, None, || {
        stat(&path).unwrap(); // "user" can stat it again
    });
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
crate::test_case! {
    /// ACL_READ_ATTRIBUTES denied prevents a user from reading file attributes
    // granular/01.t
    denied, serialized, root, FileSystemFeature::Nfsv4Acls
}
#[cfg(any(target_os = "macos", target_os = "freebsd"))]
fn denied(ctx: &mut SerializedTestContext) {
    let path = ctx.new_file(FileType::Regular).mode(0o644).create().unwrap();
    let user = ctx.get_new_user();
    ctx.as_user(&user, None, || {
        stat(&path).unwrap(); // Anybody can stat it
    });
    prependacl(&path, &format!("deny::user:{}:readattr", user.uid));
    stat(&path).unwrap(); // Owner can still stat it

    ctx.as_user(&user, None, || {
        let e = stat(&path).unwrap_err(); // "user" can no longer stat it
        assert_eq!(Errno::EACCES, e);
    });
}
