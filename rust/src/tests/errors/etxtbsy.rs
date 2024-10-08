/// Create a test case which asserts that it returns ETXTBSY
/// when the file is a pure procedure (shared text) file that is being executed.
/// There are multiple forms for this macro:
///
/// - A basic form which takes the syscall, and optionally a `~path` argument
///   to indicate where the `path` argument should be substituted if the path
///   is not the only argument taken by the syscall.
///
/// ```
/// // `unlink` accepts only a path as argument.
/// etxtbsy_test_case!(unlink);
/// // `truncate` takes a path and the flags to set as arguments.
/// // We need to add `~path` where the path argument should normally be taken.
/// etxtbsy_test_case!(truncate(~path, 123));
/// ```
///
/// - A more complex form which takes multiple functions
///   with the context and the path as arguments for syscalls
///   requring to compute other arguments.
///
/// ```
/// etxtbsy_test_case!(chown, |ctx: &mut TestContext, path: &Path| {
///   let user = ctx.get_new_user();
///   chown(path, Some(user.uid), None)
/// })
/// ```
macro_rules! etxtbsy_test_case {
    ($syscall: ident, $($f: expr),+) => {
        crate::test_case! {
            #[doc = concat!(stringify!($syscall),
                 " returns ETXTBSY when the file is a pure procedure (shared text) file that is being executed.")]
            etxtbsy; $crate::tests::errors::etxtbsy::exec_mounted
        }
        fn etxtbsy (ctx: &mut crate::context::TestContext) {
            use std::{fs::File, process::Command};

            use nix::{errno::Errno, sys::stat::Mode};

            use crate::utils::chmod;

            let sleep_path =
                String::from_utf8(Command::new("which").arg("sleep").output().unwrap().stdout).unwrap();
            let sleep_path = sleep_path.trim();

            let exec_path = ctx.gen_path();
            std::io::copy(
                &mut File::open(sleep_path).unwrap(),
                &mut File::create(&exec_path).unwrap(),
            )
            .unwrap();

            chmod(&exec_path, Mode::from_bits_truncate(0o755)).unwrap();

            let mut sleep_process = Command::new(&exec_path).arg("10").spawn().unwrap();
            $( assert_eq!($f(&exec_path).unwrap_err(), Errno::ETXTBSY); )+

            sleep_process.kill().unwrap();
        }
    };

    ($syscall: ident $( ($( $($before:expr),* ,)? ~path $(, $($after:expr),*)?) )?) => {
        etxtbsy_test_case!($syscall, |path: &std::path::Path| {
                $syscall($( $($($before),* ,)? )? path $( $(, $($after),*)? )?)
        });
    };
}

pub(crate) use etxtbsy_test_case;

use crate::config::Config;
use std::path::Path;

/// Guard which checks if a file system is mounted with the `noexec` flag.
pub fn exec_mounted(_: &Config, base_path: &Path) -> anyhow::Result<()> {
    use nix::sys::statfs::statfs;

    let flags = statfs(base_path)?.flags();

    #[cfg(target_os = "linux")]
    {
        use nix::sys::statvfs::FsFlags;

        if flags.contains(FsFlags::ST_NOEXEC) {
            anyhow::bail!("The filesystem is mounted with the noexec flag");
        }
    }

    #[cfg(not(target_os = "linux"))]
    {
        use nix::mount::MntFlags;

        if flags.contains(MntFlags::MNT_NOEXEC) {
            anyhow::bail!("The filesystem is mounted with the noexec flag");
        }
    }

    Ok(())
}
