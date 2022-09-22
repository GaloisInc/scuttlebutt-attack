#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use core::cell::RefCell;
use rand::SeedableRng;
use scuttlebutt_attack::io_merged;
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
