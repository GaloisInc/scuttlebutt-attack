#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use scuttlebutt_attack::kernel;

#[cfg(feature = "microram")]
#[no_mangle]
pub unsafe extern "C" fn __cc_syscall_handler(
    num: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> usize {
    kernel::syscall::<1>(num, arg0, arg1, arg2)
}

#[cfg(not(feature = "microram"))]
fn main() {}
