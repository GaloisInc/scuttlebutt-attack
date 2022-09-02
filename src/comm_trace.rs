use core::ops::Range;

pub enum EventKind {
    Send,
    Recv,
}

pub struct Event {
    pub thread_id: usize,
    pub channel_id: usize,
    pub kind: EventKind,
    pub range: Range<usize>,
    pub next_event_for_thread: usize,
}

pub struct Channel {
    pub start: usize,
}

pub struct Thread {
    pub first_event: usize,
}


/// Check that the trace is internally consistent.
pub fn check_trace<
    const NUM_CHANNELS: usize,
    const NUM_THREADS: usize,
>(
    threads: &[Thread; NUM_THREADS],
    channels: &[Channel; NUM_CHANNELS],
    events: &[Event],
    data_len: usize,
) {
    // `send_pos` stores the `range.end` of the most recent `Send` on each channel.
    let mut send_pos = [0; NUM_CHANNELS];
    // `recv_pos` stores the `range.end` of the most recent `Recv` on each channel.
    let mut recv_pos = [0; NUM_CHANNELS];

    for (channel_id, channel) in channels.iter().enumerate() {
        // Each entry in `channel_starts` must have a `start` that falls within the bounds of the data
        // array.  This can include `channel.start == data_len`, which is one way of indicating a
        // channel that didn't transfer any data.
        assert!(channel.start <= data_len);

        // We initialize both `send_pos` and `recv_pos` to `channel.start` so we can be sure that
        // the first send and first recv on the channel start at the same offset.
        send_pos[channel_id] = channel.start;
        recv_pos[channel_id] = channel.start;
    }

    // The index where we expect to see the next event for each thread.  If we see an event for the
    // thread at some other index, the communication trace is invalid.
    let mut expected_next_event_for_thread = [0; NUM_THREADS];

    for (thread_id, thread) in threads.iter().enumerate() {
        expected_next_event_for_thread[thread_id] = thread.first_event;
    }

    for (i, event) in events.iter().enumerate() {
        // Each `Event`'s `thread_id` must be in the range `0 .. NUM_THREADS`.
        assert!(event.thread_id < NUM_THREADS);
        // Each `Event`'s `channel_id` must be in the range `0 .. NUM_CHANNELS`.
        assert!(event.channel_id < NUM_CHANNELS);
        // Each `Event`s `range` must cover some subset of the data array.
        assert!(event.range.start <= data_len);
        assert!(event.range.end <= data_len);
        // Each `Event`s `range` must not run backwards (must have `start <= end`).
        assert!(event.range.start <= event.range.end);

        match event.kind {
            EventKind::Send => {
                // All `Send`s on a given channel must be sorted and contiguous, starting at
                // position `channel_start`.  Specifically, each `Event` must have `range.start`
                // equal to the `range.end` of the most recent operation with the same `channel_id`
                // and `kind`.  If there is no previous range, then `range.start` must equal
                // `channel_start`.
                //
                // Here, we use separate `send_pos` and `recv_pos` arrays to track the end of the
                // most recent range for each `EventKind` and channel.  Both arrays are initialized
                // to `channel_starts` to handle the case where there is no previous range.
                assert_eq!(event.range.start, send_pos[event.channel_id]);
                send_pos[event.channel_id] = event.range.end;
            },

            EventKind::Recv => {
                // `Recv`s can only return as much data as was previously sent on the channel.
                // Specifically, each `Event` of knid `Recv` must have `range.end` that does not
                // exceed the `range.end` of the most recent `Send` event with the same
                // `channel_id`.
                assert!(event.range.end <= send_pos[event.channel_id]);

                // Same conditions as for `Send`.
                assert_eq!(event.range.start, recv_pos[event.channel_id]);
                recv_pos[event.channel_id] = event.range.end;
            },
        }

        // Check that we expected to see an event for this thread at this point.  If we see an
        // event for this thread at an unexpected event index, that means a previous
        // `next_event_for_thread` index (or the initial `thread.first_event`) was incorrect.
        assert_eq!(i, expected_next_event_for_thread[event.thread_id]);
        expected_next_event_for_thread[event.thread_id] = event.next_event_for_thread;
    }

    // Check that no more events are expected for any threads.  If this is not the case, then the
    // last `next_event_for_thread` was incorrect.
    for thread_id in 0 .. NUM_THREADS {
        assert_eq!(expected_next_event_for_thread[thread_id], events.len());
    }
}
