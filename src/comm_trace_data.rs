use crate::comm_trace;
use crate::comm_trace_types::{Event, Channel, Thread};
use crate::comm_trace_types::{NUM_EVENTS, NUM_CHANNELS, NUM_THREADS, NUM_DATA_BYTES};

// The actual data is in a separate crate, exposed via `extern "Rust"` `static`s, to ensure that
// neither rustc nor LLVM can use the secret values while optimizing.
extern "Rust" {
    static CC_SSB_EVENTS: [Event; NUM_EVENTS];
    static CC_SSB_NUM_VALID_EVENTS: usize;
    static CC_SSB_CHANNELS: [Channel; NUM_CHANNELS];
    static CC_SSB_THREADS: [Thread; NUM_THREADS];
    static CC_SSB_DATA: [u8; NUM_DATA_BYTES];
}

pub fn events() -> &'static [Event] {
    unsafe {
        &CC_SSB_EVENTS[..CC_SSB_NUM_VALID_EVENTS]
    }
}

pub fn channels() -> &'static [Channel; NUM_CHANNELS] {
    unsafe {
        &CC_SSB_CHANNELS
    }
}

pub fn threads() -> &'static [Thread; NUM_THREADS] {
    unsafe {
        &CC_SSB_THREADS
    }
}

pub fn data() -> &'static [u8; NUM_DATA_BYTES] {
    unsafe {
        &CC_SSB_DATA
    }
}

pub fn check_trace() {
    comm_trace::check_trace(channels(), threads(), events(), NUM_DATA_BYTES);
}
