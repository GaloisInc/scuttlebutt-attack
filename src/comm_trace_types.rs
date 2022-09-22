use core::ops::Range;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
#[repr(C)]
pub enum EventKind {
    Send,
    Recv,
}

#[repr(C)]
pub struct Event {
    pub thread_id: usize,
    pub channel_id: usize,
    pub kind: EventKind,
    pub range: Range<usize>,
    pub next_event_for_thread: usize,
}

#[repr(C)]
pub struct Channel {
    pub start: usize,
}

#[repr(C)]
pub struct Thread {
    pub first_event: usize,
}

pub const NUM_CHANNELS: usize = 2;
pub const NUM_THREADS: usize = 2;
pub const NUM_EVENTS: usize = 16;
pub const NUM_DATA_BYTES: usize = 512;
