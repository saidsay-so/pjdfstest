use nix::unistd::chown;

use crate::{context::TestContext, utils::lchown};

use super::errors::efault::efault_path_test_case;
use super::errors::eloop::{eloop_comp_test_case, eloop_final_comp_test_case};
use super::errors::enametoolong::{enametoolong_comp_test_case, enametoolong_path_test_case};
use super::errors::enoent::{
    enoent_comp_test_case, enoent_named_file_test_case, enoent_symlink_named_file_test_case,
};
use super::errors::enotdir::enotdir_comp_test_case;
use super::errors::erofs::erofs_named_test_case;

fn chown_wrapper(ctx: &mut TestContext, path: &std::path::Path) -> nix::Result<()> {
    let user = ctx.get_new_user();
    chown(path, Some(user.uid), None)
}

// chown/01.t
enotdir_comp_test_case!(chown, chown_wrapper);

// chown/02.t
enametoolong_comp_test_case!(chown, chown_wrapper);

// chown/03.t
enametoolong_path_test_case!(chown, chown_wrapper);

// chown/04.t
enoent_named_file_test_case!(chown, chown_wrapper);

// chown/04.t
enoent_comp_test_case!(chown, chown_wrapper);

// chown/04.t
enoent_symlink_named_file_test_case!(chown, chown_wrapper);

// chown/06.t
eloop_comp_test_case!(chown, chown_wrapper);

// chown/06.t
eloop_final_comp_test_case!(chown, chown_wrapper);

// chown/09.t
erofs_named_test_case!(chown, chown_wrapper);

// chown/10.t
efault_path_test_case!(chown, |ptr| nix::libc::chown(ptr, 0, 0));

mod lchown {
    use std::path::Path;

    use super::*;

    fn lchown_wrapper<P: AsRef<Path>>(ctx: &mut TestContext, path: P) -> nix::Result<()> {
        let path = path.as_ref();
        let user = ctx.get_new_user();
        lchown(path, Some(user.uid), Some(user.gid))
    }

    // chown/01.t
    enotdir_comp_test_case!(lchown, lchown_wrapper);

    // chown/04.t
    enoent_named_file_test_case!(lchown, lchown_wrapper);
    enoent_comp_test_case!(lchown, lchown_wrapper);

    // chown/06.t#L25
    eloop_comp_test_case!(lchown, lchown_wrapper);

    // chown/02.t
    enametoolong_comp_test_case!(lchown, lchown_wrapper);

    // chown/03.t
    enametoolong_path_test_case!(lchown, lchown_wrapper);

    // chown/09.t
    erofs_named_test_case!(lchown, lchown_wrapper);

    // chown/10.t
    efault_path_test_case!(lchown, |ptr| nix::libc::lchown(ptr, 0, 0));
}
