use core::cell::RefCell;
use rand::SeedableRng;
use scuttlebutt_attack::attacker;
use scuttlebutt_attack::comm_trace::ThreadState;
use scuttlebutt_attack::comm_trace_data;
use scuttlebutt_attack::io_merged::MergedChannel;
use scuttlebutt_attack::util::ChannelPair;

fn main() {
    let state = RefCell::new(ThreadState::new(
        comm_trace_data::events(),
        &comm_trace_data::threads()[1],
    ));

    let to_server = MergedChannel::new(&state, 1);
    let from_server = MergedChannel::new(&state, 0);
    let channel = ChannelPair(to_server, from_server);
    attacker::run(channel).unwrap();
    eprintln!("attacker: attack succeeded");
}
