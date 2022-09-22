#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use scuttlebutt_attack::attacker;
use scuttlebutt_attack::io_kernel::{self, KernelChannel};
use scuttlebutt_attack::kernel;
use scuttlebutt_attack::util::ChannelPair;

#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    // In the MicroRAM build, the kernel initializes itself.  In the native build, the kernel is
    // linked into the program, and we need to initialize it at the start of main().
    #[cfg(not(feature = "microram"))]
    unsafe { kernel::init_thread_state(1) };

    let to_server = KernelChannel::new(1);
    let from_server = KernelChannel::new(0);
    let channel = ChannelPair(to_server, from_server);
    attacker::run(channel).unwrap();
    io_kernel::exit();
}
