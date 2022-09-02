use crate::comm_trace::{Event, EventKind, Channel, Thread};

pub const NUM_CHANNELS: usize = 2;
pub const NUM_THREADS: usize = 2;
pub const NUM_EVENTS: usize = 16;

// The actual data is in a separate crate, exposed via `extern "Rust"` `static`s, to ensure that
// neither rustc nor LLVM can use the secret values while optimizing.
extern "Rust" {
    static CC_SSB_EVENTS: [Event; NUM_EVENTS];
    static CC_SSB_NUM_VALID_EVENTS: usize;
    static CC_SSB_CHANNELS: [Channel; NUM_CHANNELS];
    static CC_SSB_THREADS: [Thread; NUM_THREADS];
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
