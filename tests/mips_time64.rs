#![cfg(all(target_env = "gnu", gnu_time_bits64, any(target_arch = "mips", target_arch = "mips32r6")))]
#[test]
fn time64_fields_follow_native_layout() {
    use core::mem::offset_of;
    use libc::{msqid_ds, stat, stat64};
    let ns = if cfg!(target_endian = "big") { 12 } else { 8 };
    assert_eq!(offset_of!(stat, st_atime_nsec) - offset_of!(stat, st_atime), ns);
    assert_eq!(offset_of!(stat64, st_atime_nsec) - offset_of!(stat64, st_atime), ns);
    assert_eq!(offset_of!(msqid_ds, msg_qnum) - offset_of!(msqid_ds, msg_ctime), 12);
}
