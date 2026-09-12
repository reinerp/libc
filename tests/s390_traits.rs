#![cfg(all(target_arch = "s390x", target_env = "musl", feature = "extra_traits"))]

#[test]
fn register_union_has_no_hash() {
    trait Ambiguous<A> { fn marker() {} }
    impl<T: ?Sized> Ambiguous<()> for T {}
    impl<T: ?Sized + core::hash::Hash> Ambiguous<u8> for T {}
    let _ = <libc::fpreg_t as Ambiguous<_>>::marker;
}
