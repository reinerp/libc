#![cfg(all(target_os = "linux", target_arch = "riscv32"))]
use core::mem::{align_of, offset_of, size_of};

#[test]
fn ipc_mode_width() {
    let _: fn(libc::ipc_perm) -> u32 = |p| p.mode;
}

#[test]
#[cfg(target_env = "gnu")]
fn gnu_layouts() {
    assert_eq!(
        (size_of::<libc::stat>(), size_of::<libc::stat64>()),
        (128, 128)
    );
    assert_eq!(offset_of!(libc::stat, st_ino), 8);
    assert_eq!(offset_of!(libc::statvfs64, f_flag), 64);
    assert_eq!(
        offset_of!(libc::semid_ds, sem_ctime) - offset_of!(libc::semid_ds, sem_otime),
        8
    );
    assert_eq!(
        (
            size_of::<libc::pthread_mutex_t>(),
            size_of::<libc::pthread_rwlock_t>(),
            size_of::<libc::pthread_barrier_t>(),
            size_of::<libc::pthread_attr_t>()
        ),
        (32, 48, 20, 32)
    );
}

#[test]
#[cfg(target_env = "musl")]
fn musl_layouts() {
    assert_eq!(size_of::<libc::time_t>(), 8);
    assert_eq!(offset_of!(libc::msqid_ds, msg_stime), 88);
    assert_eq!(offset_of!(libc::shmid_ds, shm_atime), 88);
    assert_eq!(
        (
            size_of::<libc::max_align_t>(),
            align_of::<libc::max_align_t>()
        ),
        (32, 16)
    );
    let _: fn(libc::ipc_perm) -> i32 = |p| p.__seq;
}
