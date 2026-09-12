#[cfg(all(target_os = "hurd", target_pointer_width = "64"))]
#[test]
fn socket_storage_has_native_alignment() {
    assert_eq!(core::mem::align_of::<libc::sockaddr_storage>(), 8);
    assert_eq!(core::mem::size_of::<libc::sockaddr_storage>(), 128);
}

#[cfg(target_os = "fuchsia")]
#[test]
fn next_control_header_uses_bytes() {
    unsafe {
        let mut storage = [0u64; 128];
        let first = storage.as_mut_ptr().cast::<libc::cmsghdr>();
        let next = storage.as_mut_ptr().cast::<u8>().add(16).cast::<libc::cmsghdr>();
        (*first).cmsg_len = 16;
        let mut message = core::mem::zeroed::<libc::msghdr>();
        message.msg_control = first.cast(); message.msg_controllen = 32;
        assert_eq!(libc::CMSG_NXTHDR(&message, first), next);
    }
}

#[cfg(all(target_os = "nto", target_arch = "x86_64"))]
const _: fn() = || {
    trait Absent<A> { fn check() {} }
    impl<T: ?Sized> Absent<()> for T {}
    struct HasEq; impl<T: ?Sized + Eq> Absent<HasEq> for T {}
    struct HasHash; impl<T: ?Sized + core::hash::Hash> Absent<HasHash> for T {}
    let _ = <libc::x86_64_fpu_registers as Absent<_>>::check;
};
