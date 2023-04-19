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


#[no_mangle]
#[link_section = ".rodata.secret.__commitment_randomness__"]
pub static CC_COMMITMENT_RANDOMNESS: [u8; 32] = [
    // BEGIN commitment randomness
    // AUTO-GENERATED - DO NOT EDIT
    // Generated by update_commitment.py at Wed Apr 19 16:07:44 2023
    0xae, 0x3b, 0x86, 0xbb, 0xb5, 0x99, 0x10, 0x6b,
    0x37, 0xb9, 0x7b, 0xc9, 0xca, 0x4f, 0x08, 0x8b,
    0xb7, 0x8d, 0x61, 0x84, 0x7f, 0x5f, 0x91, 0x20,
    0x04, 0x13, 0x60, 0x4f, 0x48, 0x6e, 0x73, 0xeb,
    // END commitment randomness
];


// In the native build, `__cc_syscall` dispatches directly to `kernel::syscall` with the
// appropriate `THREAD_ID`.  In the MicroRAM build, `__cc_syscall` is an intrinsic, which the
// MicroRAM compiler hooks up to the `__cc_syscall_handler` function in `bin/kernel_attacker.rs`.
#[cfg(not(feature = "microram"))]
#[no_mangle]
pub unsafe extern "C" fn __cc_syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    scuttlebutt_attack::kernel::syscall::<1>(num, arg0, arg1, arg2)
}
