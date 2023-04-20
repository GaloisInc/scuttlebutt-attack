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
    // Placeholder value.  `cheesecloth/scripts/run_scuttlebutt` will overwrite this value in the
    // CBOR's initial memory using `commitment_tool --set-randomness`.
    //
    // The value isn't used for anything, so we don't need to worry about it being inlined.
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];


// In the native build, `__cc_syscall` dispatches directly to `kernel::syscall` with the
// appropriate `THREAD_ID`.  In the MicroRAM build, `__cc_syscall` is an intrinsic, which the
// MicroRAM compiler hooks up to the `__cc_syscall_handler` function in `bin/kernel_attacker.rs`.
#[cfg(not(feature = "microram"))]
#[no_mangle]
pub unsafe extern "C" fn __cc_syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    scuttlebutt_attack::kernel::syscall::<1>(num, arg0, arg1, arg2)
}
