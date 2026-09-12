#![cfg(any(target_os = "linux", target_os = "android"))]
#![allow(deprecated)]

#[test]
fn flexible_arrays_do_not_hash_trailing_memory() {
    trait Ambiguous<A> {
        fn check() {}
    }
    impl<T> Ambiguous<()> for T {}
    impl<T: core::hash::Hash> Ambiguous<u8> for T {}
    let _ = <libc::af_alg_iv as Ambiguous<_>>::check;
}
