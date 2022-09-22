#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use core::cell::RefCell;
use rand::SeedableRng;
use scuttlebutt_attack::comm_trace::ThreadState;
use scuttlebutt_attack::comm_trace_data;
use scuttlebutt_attack::io_merged::{self, MergedChannel};
use scuttlebutt_attack::server;
use scuttlebutt_attack::util::ChannelPair;

#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    let mut rng = rand_chacha::ChaCha12Rng::from_seed(scuttlebutt_attack_constants::SEED);

    let state = RefCell::new(ThreadState::new(
        comm_trace_data::events(),
        &comm_trace_data::threads()[0],
    ));

    let to_client = MergedChannel::new(&state, 0);
    let from_client = MergedChannel::new(&state, 1);
    let channel = ChannelPair(to_client, from_client);
    server::run(&mut rng, channel).unwrap();
    io_merged::exit();
}
