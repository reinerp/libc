#![cfg(target_os = "android")]

#[test]
fn cpu_allocation_rounds_to_words() {
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(0) }, 0);
    assert_eq!(unsafe { libc::CPU_ALLOC_SIZE(64) }, 8);
}
