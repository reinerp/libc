#![cfg(all(target_os = "linux", target_env = "gnu", gnu_time_bits64))]

#[test]
fn timespec_has_native_time64_layout() {
    assert_eq!(core::mem::size_of::<libc::timespec>(), 16);
    let offset = if cfg!(target_endian = "big") { 12 } else { 8 };
    assert_eq!(core::mem::offset_of!(libc::timespec, tv_nsec), offset);
}

#[test]
#[cfg(target_arch = "arm")]
fn stat64_nanoseconds_follow_endianness() {
    let offset = if cfg!(target_endian = "big") { 12 } else { 8 };
    assert_eq!(
        core::mem::offset_of!(libc::stat64, st_atime_nsec)
            - core::mem::offset_of!(libc::stat64, st_atime),
        offset
    );
}
