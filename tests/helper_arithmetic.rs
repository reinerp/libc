#![cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "musl")))]

#[test]
fn cpu_allocation_rounds_to_words() {
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(0) }, 0);
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(64) }, 8);
}

#[test]
fn elf32_relocation_round_trip() {
    let info = unsafe { libc::ELF32_R_INFO(0x123, 7) };
    assert_eq!(
        unsafe { (libc::ELF32_R_SYM(info), libc::ELF32_R_TYPE(info)) },
        (0x123, 7)
    );
}

#[test]
fn elf64_relocation_round_trip() {
    let info = unsafe { libc::ELF64_R_INFO(0x123, 7) };
    assert_eq!(
        unsafe { (libc::ELF64_R_SYM(info), libc::ELF64_R_TYPE(info)) },
        (0x123, 7)
    );
}

#[test]
fn unix_socket_length_includes_family() {
    let mut addr = libc::sockaddr_un {
        sun_family: libc::AF_UNIX as _,
        sun_path: [0; 108],
    };
    addr.sun_path[..3].copy_from_slice(&[b'a' as _, b'b' as _, b'c' as _]);
    assert_eq!(unsafe { libc::SUN_LEN(addr) }, 5);
}
