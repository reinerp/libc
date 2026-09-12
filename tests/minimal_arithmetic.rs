#[cfg(any(target_os = "android", all(target_os = "linux", any(target_env = "gnu", target_env = "musl"))))]
#[test]
fn cpu_allocation_rounds_to_words() {
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(0) }, 0);
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(64) }, 8);
}

#[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "musl")))]
#[test]
fn elf_relocation_round_trip() {
    let info = unsafe { libc::ELF32_R_INFO(0x123, 7) };
    assert_eq!(unsafe { (libc::ELF32_R_SYM(info), libc::ELF32_R_TYPE(info)) }, (0x123, 7));
    let info = unsafe { libc::ELF64_R_INFO(0x123, 7) };
    assert_eq!(unsafe { (libc::ELF64_R_SYM(info), libc::ELF64_R_TYPE(info)) }, (0x123, 7));
}

#[cfg(all(target_os = "linux", target_env = "musl"))]
#[test]
fn unix_socket_length_includes_family() {
    let mut addr = libc::sockaddr_un { sun_family: libc::AF_UNIX as _, sun_path: [0; 108] };
    addr.sun_path[..3].copy_from_slice(&[b'a' as _, b'b' as _, b'c' as _]);
    assert_eq!(unsafe { libc::SUN_LEN(addr) }, 5);
}

#[cfg(target_os = "vxworks")]
#[test]
fn zero_alignment_is_invalid() {
    let mut pointer = core::ptr::null_mut();
    assert_eq!(unsafe { libc::posix_memalign(&mut pointer, 0, 1) }, libc::EINVAL);
    assert!(pointer.is_null());
}
