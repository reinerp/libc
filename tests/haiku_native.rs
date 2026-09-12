#![cfg(target_os = "haiku")]

#[test]
fn cpuid_record_is_four_registers() {
    assert_eq!(core::mem::size_of::<libc::cpuid_info>(), 16);
    assert_eq!(core::mem::size_of::<libc::__c_anonymous_eax_1>(), 16);
    assert_eq!(
        core::mem::offset_of!(libc::__c_anonymous_eax_1, features),
        8
    );
}

const _: fn(libc::image_info) -> Option<extern "C" fn()> = |i| i.init_routine;
const _: fn(libc::image_info) -> Option<extern "C" fn()> = |i| i.term_routine;

#[cfg(feature = "extra_traits")]
const _: fn() = || {
    trait Ambiguous<A> {
        fn check() {}
    }
    impl<T: ?Sized> Ambiguous<()> for T {}
    impl<T: ?Sized + Eq> Ambiguous<u8> for T {}
    impl<T: ?Sized + core::hash::Hash> Ambiguous<u16> for T {}
    let _ = <libc::cpuid_info as Ambiguous<_>>::check;
    let _ = <libc::__c_anonymous_cpu_topology_info_data as Ambiguous<_>>::check;
    let _ = <libc::cpu_topology_node_info as Ambiguous<_>>::check;
};
