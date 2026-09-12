#![cfg(target_os = "cygwin")]
use libc::*;

#[test]
fn cpu_sets_span_all_words() {
    let mut set = unsafe { core::mem::zeroed::<cpu_set_t>() };
    unsafe {
        CPU_SET(127, &mut set);
        assert!(CPU_ISSET(127, &set));
        CPU_CLR(127, &mut set);
        assert_eq!(CPU_COUNT(&set), 0);
        assert_eq!(CPU_ALLOC_SIZE(128), 16);
    }
}

#[test]
fn uname_layout() {
    assert_eq!(core::mem::size_of::<utsname>(), 390);
    assert_eq!(core::mem::offset_of!(utsname, nodename), 65);
}
