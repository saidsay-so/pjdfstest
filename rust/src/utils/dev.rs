// TODO: Would be clearer with cfg_if

#[cfg(target_os = "freebsd")]
pub use freebsd::*;
#[cfg(target_os = "linux")]
pub use linux::*;
#[cfg(target_os = "solaris")]
pub use solaris::*;

#[cfg(target_os = "linux")]
mod linux {
    pub fn makedev(major: u64, minor: u64) -> nix::libc::dev_t {
        nix::sys::stat::makedev(major, minor)
    }

    pub fn major(dev: nix::libc::dev_t) -> u64 {
        nix::sys::stat::major(dev)
    }

    pub fn minor(dev: nix::libc::dev_t) -> u64 {
        nix::sys::stat::minor(dev)
    }
}

// TODO: Upstream to nix/libc?

#[cfg(target_os = "freebsd")]
mod freebsd {
    // sys/types.h
    // TODO: Replace u64 by c_int?

    pub fn makedev(major: u64, minor: u64) -> nix::libc::dev_t {
        (((major & 0xffffff00) << 32)
            | ((major & 0x000000ff) << 8)
            | ((minor & 0xff00) << 24)
            | minor & 0xffff00ff)
            .try_into()
            .unwrap()
    }

    pub fn major(dev: u64) -> u64 {
        (((dev >> 32) & 0xffffff00) | ((dev >> 8) & 0xff)).into()
    }

    pub fn minor(dev: u64) -> u64 {
        (((dev >> 24) & 0xff00) | (dev & 0xffff00ff)).into()
    }
}

#[cfg(target_os = "solaris")]
mod solaris {
    // sysmacros.h

    type major_t = nix::libc::uint32_t;
    type minor_t = nix::libc::uint32_t;

    mod x32 {
        const L_BITSMAJOR: usize = 14; /* # of SVR4 major device bits */
        const L_BITSMINOR: usize = 18; /* # of SVR4 minor device bits */
        const L_MAXMAJ: usize = 0x3fff; /* SVR4 max major value */
        const L_MAXMIN: usize = 0x3ffff; /* MAX minor for 3b2 software drivers. */
    }

    mod x64 {
        const L_BITSMAJOR: usize = 32; /* # of major device bits in 64-bit Solaris */
        const L_BITSMINOR: usize = 32; /* # of minor device bits in 64-bit Solaris */
        const L_MAXMAJ: usize = 0xffffffff; /* max major value */
        const L_MAXMIN: usize = 0xffffffff; /* max minor value */
    }

    #[cfg(target_pointer_width = "32")]
    use x32::*;
    #[cfg(target_pointer_width = "64")]
    use x64::*;

    pub fn makedev(major: major_t, minor: minor_t) -> nix::libc::dev_t {
        ((major) << L_BITSMINOR) | ((minor) & L_MAXMIN)
    }

    pub fn major(dev: nix::libc::dev_t) -> major_t {
        (dev >> L_BITSMAJOR)
    }

    pub fn minor(dev: nix::libc::dev_t) -> minor_t {
        (dev & L_MAXMIN)
    }
}
