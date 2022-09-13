#![no_std]
#![no_main]
#![feature(lang_items)]
use core::cell::RefCell;
use core::panic::PanicInfo;
use rand::SeedableRng;
use scuttlebutt_attack::comm_trace::ThreadState;
use scuttlebutt_attack::comm_trace_data;
use scuttlebutt_attack::io_merged::MergedChannel;
use scuttlebutt_attack::server;
use scuttlebutt_attack::util::ChannelPair;

#[no_mangle]
pub fn main() {
    // TODO: seed rng using fiat-shamir
    let mut rng = rand_chacha::ChaCha12Rng::from_seed([77; 32]);

    let state = RefCell::new(ThreadState::new(
        comm_trace_data::events(),
        &comm_trace_data::threads()[0],
    ));

    let to_client = MergedChannel::new(&state, 0);
    let from_client = MergedChannel::new(&state, 1);
    let channel = ChannelPair(to_client, from_client);
    server::run(&mut rng, channel).unwrap();
    // TODO: report success
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    extern "C" {
        fn __cc_answer(code: i32) -> !;
    }
    unsafe {
        __cc_answer(0);
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
