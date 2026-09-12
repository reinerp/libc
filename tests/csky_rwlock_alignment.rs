#![cfg(target_arch = "csky")]

#[test]
fn rwlock_alignment() {
    assert_eq!(core::mem::align_of::<libc::pthread_rwlock_t>(), 4);
}
