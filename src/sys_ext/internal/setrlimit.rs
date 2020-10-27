pub use libc::rlimit64 as Rlimit;

use nix::errno::Errno;

#[cfg(not(any(target_env = "musl")))]
pub fn setrlimit(resource: libc::c_uint, rlimit: &Rlimit) -> nix::Result<()> {
    let res = unsafe { libc::setrlimit64(resource, rlimit as *const Rlimit) };
    Errno::result(res).map(drop)
}

#[cfg(any(target_env = "musl"))]
pub fn setrlimit(resource: libc::c_int, rlimit: &Rlimit) -> nix::Result<()> {
    let res = unsafe { libc::setrlimit64(resource, rlimit as *const Rlimit) };
    Errno::result(res).map(drop)
}
