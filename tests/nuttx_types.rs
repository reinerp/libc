#![cfg(target_os = "nuttx")]

#[test]
fn native_types() {
    let _: libc::dev_t = u32::MAX;
    let _: fn(libc::stat) -> libc::blkcnt_t = |stat| stat.st_blocks;
    let _: unsafe extern "C" fn(
        i32,
        *mut libc::c_void,
        usize,
        i32,
        *mut libc::sockaddr,
        *mut libc::socklen_t,
    ) -> libc::ssize_t = libc::recvfrom;
}
