#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use scuttlebutt_attack::attacker;
use scuttlebutt_attack::io_kernel::{self, KernelChannel};
use scuttlebutt_attack::util::ChannelPair;

#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    let to_server = KernelChannel::new(1);
    let from_server = KernelChannel::new(0);
    let channel = ChannelPair(to_server, from_server);
    attacker::run(channel).unwrap();
    io_kernel::exit();
}

// In the native build, `__cc_syscall` dispatches directly to `kernel::syscall` with the
// appropriate `THREAD_ID`.  In the MicroRAM build, `__cc_syscall` is an intrinsic, which the
// MicroRAM compiler hooks up to the `__cc_syscall_handler` function in `bin/kernel_attacker.rs`.
#[cfg(not(feature = "microram"))]
#[no_mangle]
pub unsafe extern "C" fn __cc_syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    scuttlebutt_attack::kernel::syscall::<1>(num, arg0, arg1, arg2)
}
