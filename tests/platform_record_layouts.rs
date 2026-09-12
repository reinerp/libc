#[cfg(all(target_os = "linux", target_env = "musl", target_arch = "hexagon"))]
#[test]
fn shared_memory_record_fits_native_output() {
    assert_eq!(core::mem::size_of::<libc::shmid_ds>(), 112);
}
#[cfg(all(target_os = "linux", target_env = "gnu", target_arch = "sparc", not(gnu_file_offset_bits64)))]
#[test]
fn file_lock_fits_native_output() {
    assert_eq!(core::mem::size_of::<libc::flock>(), 20);
}
#[cfg(all(target_os = "emscripten", target_pointer_width = "64"))]
#[test]
fn native_lock_alignment() {
    assert_eq!(core::mem::align_of::<libc::pthread_mutex_t>(), 8);
    assert_eq!(core::mem::align_of::<libc::pthread_rwlock_t>(), 8);
}

#[cfg(target_os = "hurd")]
#[test]
fn received_descriptors_can_be_close_on_exec() {
    assert_eq!(libc::MSG_CMSG_CLOEXEC, 0x40000);
}

#[cfg(all(any(target_os = "hurd", target_os = "emscripten"), target_pointer_width = "64"))]
#[test]
fn final_control_header_fits() {
    unsafe {
        let mut storage = [0u64; 8];
        let first = storage.as_mut_ptr().cast::<libc::cmsghdr>();
        let next = storage.as_mut_ptr().cast::<u8>().add(16).cast::<libc::cmsghdr>();
        (*first).cmsg_len = 12; (*next).cmsg_len = 12;
        let mut message = core::mem::zeroed::<libc::msghdr>();
        message.msg_control = first.cast(); message.msg_controllen = 28;
        assert_eq!(libc::CMSG_NXTHDR(&message, first), next);
    }
}
