#![cfg(any(target_arch = "mips32r6", target_arch = "mips64r6"))]

#[test]
fn mips_r6_constants() {
    assert_eq!(
        (libc::SI_TIMER, libc::SI_MESGQ, libc::SI_ASYNCIO),
        (-3, -4, -2)
    );
    assert_eq!(libc::_IO(0x12, 3), 0x20001203);
    assert_eq!(libc::_IOW::<u32>(0x12, 3), 0x80041203);
    assert_eq!(libc::_IOR::<u32>(0x12, 3), 0x40041203);
}
