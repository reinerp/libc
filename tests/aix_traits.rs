#![cfg(target_os = "aix")]

#[cfg(feature = "extra_traits")]
#[test]
fn fpreg_equality_matches_bits() {
    assert!(libc::fpreg_t { d: f64::NAN } == libc::fpreg_t { d: f64::NAN });
    assert!(libc::fpreg_t { d: 0.0 } != libc::fpreg_t { d: -0.0 });
}

#[test]
fn nullable_notification_callback() {
    let _: fn(&mut libc::sigevent) = |event| event.sigev_notify_function = None;
}
