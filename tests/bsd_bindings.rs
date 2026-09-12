use libc::*;

// Compile-only: a union with an unknown active member must not acquire safe traits.
macro_rules! no_eq_or_hash {
    ($ty:ty) => {
        const _: fn() = || {
            trait Ambiguous<A> {
                fn check() {}
            }
            impl<T: ?Sized> Ambiguous<()> for T {}
            impl<T: ?Sized + Eq> Ambiguous<u8> for T {}
            impl<T: ?Sized + std::hash::Hash> Ambiguous<u16> for T {}
            let _ = <$ty as Ambiguous<_>>::check;
        };
    };
}
#[cfg(target_vendor = "apple")]
no_eq_or_hash!(ifreq);
#[cfg(target_vendor = "apple")]
no_eq_or_hash!(ifkpi);
#[cfg(target_vendor = "apple")]
no_eq_or_hash!(in6_ifreq);
#[cfg(target_os = "freebsd")]
no_eq_or_hash!(if_data);
#[cfg(target_os = "freebsd")]
no_eq_or_hash!(ifreq);
#[cfg(target_os = "openbsd")]
no_eq_or_hash!(mount_info);
#[cfg(target_os = "openbsd")]
no_eq_or_hash!(ifreq);
#[cfg(target_os = "netbsd")]
no_eq_or_hash!(posix_spawn_file_actions_entry_t);

#[cfg(target_os = "netbsd")]
#[test]
fn device_major_round_trips() {
    assert_eq!(major(makedev(0xabc, 0x12345)), 0xabc);
}

#[cfg(all(
    target_os = "netbsd",
    target_arch = "riscv64",
    feature = "extra_traits"
))]
#[test]
fn floating_register_equality_is_bitwise() {
    assert_ne!(__fpreg { u_d: 0.0 }, __fpreg { u_d: -0.0 });
}

#[cfg(all(target_os = "freebsd", feature = "extra_traits"))]
#[test]
fn elf32_value_can_be_hashed() {
    use std::hash::{Hash, Hasher};
    let mut actual = std::collections::hash_map::DefaultHasher::new();
    let mut expected = actual.clone();
    __c_anonymous_elf32_auxv_union { a_val: 7 }.hash(&mut actual);
    7i32.hash(&mut expected);
    assert_eq!(actual.finish(), expected.finish());
}

#[cfg(all(target_os = "freebsd", target_arch = "aarch64"))]
const _: [(); 528] = [(); std::mem::size_of::<fpregs>()];

#[cfg(target_os = "freebsd")]
const _: devstat_support_flags = devstat_support_flags(3);
#[cfg(target_os = "freebsd")]
const _: devstat_match_flags = devstat_match_flags(3);
#[cfg(target_os = "freebsd")]
const _: devstat_type_flags = devstat_type_flags(0x101);

#[cfg(target_os = "netbsd")]
const _: unsafe extern "C" fn(*const c_char, *const c_char) -> *mut FILE = efopen;
#[cfg(target_os = "netbsd")]
const _: unsafe extern "C" fn(c_ulong, *const c_char) -> *mut c_char = flags_to_string;
#[cfg(target_os = "netbsd")]
const _: unsafe extern "C" fn(c_int, *const c_char) -> c_int = fremovexattr;
#[cfg(target_os = "netbsd")]
const _: unsafe extern "C" fn(*const c_char, *const c_char, *const c_void, size_t, c_int) -> c_int =
    setxattr;
#[cfg(target_os = "netbsd")]
const _: unsafe extern "C" fn(*const c_char, *const c_char, *const c_void, size_t, c_int) -> c_int =
    lsetxattr;

#[cfg(target_os = "dragonfly")]
const _: unsafe extern "C" fn(*const c_char) -> *mut utmpx = getutxuser;
#[cfg(target_os = "dragonfly")]
const _: unsafe extern "C" fn(*const c_char, *const utmpx) = updwtmpx;

#[cfg(target_vendor = "apple")]
const _: unsafe extern "C" fn(
    pthread_introspection_hook_t,
) -> Option<pthread_introspection_hook_t> = pthread_introspection_hook_install;
#[cfg(all(target_vendor = "apple", any(target_arch = "arm", target_arch = "x86")))]
const _: fn(mcontext_t) -> *mut __darwin_mcontext32 = |p| p;

#[cfg(target_vendor = "apple")]
#[test]
fn ioctl_sizes_follow_pointer_width() {
    let pointer = std::mem::size_of::<usize>() as c_ulong;
    for (request, size) in [
        (SIOCGIFMEDIA, 36 + pointer),
        (SIOCGIFXMEDIA, 36 + pointer),
        (SIOCRSLVMULTI, 2 * pointer),
        (SIOCIFGCLONERS, 8 + pointer),
        (SIOCSDRVSPEC, 16 + 3 * pointer),
        (SIOCGDRVSPEC, 16 + 3 * pointer),
    ] {
        assert_eq!((request >> 16) & 0x1fff, size);
    }
}
