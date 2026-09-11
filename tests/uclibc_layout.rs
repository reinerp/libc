//! Layout expectations for the CI uClibc-ng 1.0.45/1.0.54 native ABI.
#![cfg(target_env = "uclibc")]
use core::mem::{offset_of, size_of};

#[test]
#[cfg(target_arch = "x86_64")]
fn x86_64_layouts() {
    assert_eq!(libc::O_CREAT, 0o100);
    assert_eq!(
        (size_of::<libc::stat>(), offset_of!(libc::stat, st_mode)),
        (144, 24)
    );
    assert_eq!(size_of::<libc::statfs64>(), 120);
    assert_eq!(size_of::<libc::statvfs64>(), 112);
    assert_eq!(size_of::<libc::termios>(), 60);
    assert_eq!(size_of::<libc::sigset_t>(), 8);
    let _: fn(libc::ipc_perm) -> u32 = |p| p.mode;
}

#[test]
#[cfg(target_arch = "mips64")]
fn mips64_layouts() {
    assert_eq!(
        (size_of::<libc::stat>(), size_of::<libc::stat64>()),
        (192, 192)
    );
    assert_eq!(offset_of!(libc::stat, st_blksize), 120);
    assert_eq!(offset_of!(libc::stat64, st_blksize), 120);
    assert_eq!(size_of::<libc::sigset_t>(), 16);
}

#[test]
#[cfg(all(target_arch = "mips", linux_time_bits64))]
fn mips_time64_stat_padding() {
    assert_eq!(offset_of!(libc::stat, st_blksize), 112);
    assert_eq!(offset_of!(libc::stat64, st_blksize), 112);
}

#[test]
#[cfg(target_pointer_width = "32")]
fn statvfs_fsid_precedes_padding() {
    assert_eq!(offset_of!(libc::statvfs, f_fsid), 32);
}
