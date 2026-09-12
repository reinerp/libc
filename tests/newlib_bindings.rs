#![cfg(target_env = "newlib")]
use libc::*;

#[test]
fn signal_handler_sentinels() {
    let _: fn(&mut sigaction) = |action| action.sa_handler = SIG_DFL;
    #[cfg(target_os = "rtems")]
    assert_eq!(core::mem::offset_of!(sigaction, sa_flags), 0);
}

#[cfg(target_os = "espidf")]
#[test]
fn esp_native_types() {
    fn layout<T>() -> (usize, usize) {
        (core::mem::size_of::<T>(), core::mem::align_of::<T>())
    }
    assert_eq!(layout::<pthread_mutex_t>(), (4, 4));
    assert_eq!(layout::<pthread_cond_t>(), (4, 4));
    assert_eq!(layout::<pthread_rwlock_t>(), (4, 4));
    assert_eq!(layout::<pthread_rwlockattr_t>(), (4, 4));
    let _: unsafe extern "C" fn(*mut c_char, size_t) -> c_int = gethostname;
}
