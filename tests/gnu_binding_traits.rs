#![cfg(all(target_os = "linux", target_env = "gnu", feature = "extra_traits"))]

#[test]
fn ptrace_union_has_no_safe_hash() {
    trait Ambiguous<A> { fn check() {} }
    impl<T: ?Sized> Ambiguous<()> for T {}
    impl<T: ?Sized + core::hash::Hash> Ambiguous<u8> for T {}
    let _ = <libc::__c_anonymous_ptrace_syscall_info_data as Ambiguous<_>>::check;
    let _ = <libc::ptrace_syscall_info as Ambiguous<_>>::check;
}
