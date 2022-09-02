import argparse
import cbor
from collections import defaultdict
from pprint import pprint
import sys


def parse_args():
    parser = argparse.ArgumentParser(
        description='Generate trace_data.rs from parameters and optional recording')
    parser.add_argument('num_channels', type=int)
    parser.add_argument('num_threads', type=int)
    parser.add_argument('num_events', type=int)
    parser.add_argument('recording_path', metavar='recording.cbor', nargs='?')
    return parser.parse_args()


EMPTY_EVENT = {
        'thread_id': 0,
        'channel_id': 0,
        'kind': 'Recv',
        'range_start': 0,
        'range_end': 0,
        'next_event_for_thread': 0,
        }

EMPTY_CHANNEL = {
        'start': 0,
        }

EMPTY_THREAD = {
        'first_event': 0,
        }


def parse_recording(args):
    with open(args.recording_path, 'rb') as f:
        c = cbor.load(f)


    # Initial conversion.  We build up the data array for each channel, and emit
    # preliminary events with relative start/end positions and no
    # next_event_for_thread pointers.

    channel_data = defaultdict(bytearray)
    send_offset = defaultdict(int)
    recv_offset = defaultdict(int)
    events = []

    def emit_event(thread_id, channel_id, length, kind):
        if kind == 'Send':
            start = send_offset[channel_id]
            end = start + length
            send_offset[channel_id] = end
        elif kind == 'Recv':
            start = recv_offset[channel_id]
            end = start + length
            recv_offset[channel_id] = end
        else:
            raise ValueError('bad kind %r' % (kind,))

        events.append({
            'thread_id': thread_id,
            'channel_id': channel_id,
            'kind': kind,
            'range_start': start,
            'range_end': end,
            'next_event_for_thread': None,
        })

    for evt in c:
        if 'Write' in evt['kind']:
            data = evt['kind']['Write']
            channel_data[evt['channel_id']].extend(data)
            emit_event(evt['thread_id'], evt['channel_id'], len(data), 'Send')
        elif 'Read' in evt['kind']:
            length = evt['kind']['Read']
            emit_event(evt['thread_id'], evt['channel_id'], length, 'Recv')
        else:
            raise ValueError('bad kind %r' % (evt['kind'],))


    # Compute the proper next_event_for_thread index for each event.
    next_event_for_thread = defaultdict(lambda: len(events))
    for i in reversed(range(0, len(events))):
        evt = events[i]
        evt['next_event_for_thread'] = next_event_for_thread[evt['thread_id']]
        next_event_for_thread[evt['thread_id']] = i


    # Flatten channel data into a single array, and adjust each event's start/end
    # positions from relative to absolute indices.

    channel_base = {}
    all_data = bytearray()
    for channel_id, data in channel_data.items():
        channel_base[channel_id] = len(all_data)
        all_data.extend(data)

    for evt in events:
        evt['range_start'] += channel_base[evt['channel_id']]
        evt['range_end'] += channel_base[evt['channel_id']]


    # Pad out arrays to the required lengths

    assert len(events) <= args.num_events, \
            'too many events in recording: %d > %d' % (len(events), args.num_events)
    num_valid_events = len(events)
    while len(events) < args.num_events:
        events.append(EMPTY_EVENT)

    assert max(channel_base.keys()) + 1 <= args.num_channels, \
            'too many channels in recording: %d > %d' % (
                    max(channel_base.keys()) + 1, args.num_channels)
    channels = [EMPTY_CHANNEL.copy() for _ in range(args.num_channels)]
    for i, start in channel_base.items():
        channels[i] = {
            'start': start,
            }

    assert max(next_event_for_thread.keys()) + 1 <= args.num_threads, \
            'too many threads in recording: %d > %d' % (
                    max(next_event_for_thread.keys()) + 1, args.num_threads)
    threads = [EMPTY_THREAD.copy() for _ in range(args.num_threads)]
    for i, first_event in next_event_for_thread.items():
        threads[i] = {
            'first_event': first_event,
            }

    return events, num_valid_events, channels, threads

def dummy_values(args):
    events = [EMPTY_EVENT] * args.num_events
    num_valid_events = 0
    channels = [EMPTY_CHANNEL] * args.num_channels
    threads = [EMPTY_THREAD] * args.num_threads
    return events, num_valid_events, channels, threads


args = parse_args()

if args.recording_path is not None:
    events, num_valid_events, channels, threads = parse_recording(args)
else:
    events, num_valid_events, channels, threads = dummy_values(args)

# Generate code

print('use scuttlebutt_attack::comm_trace::{Event, EventKind, Channel, Thread};')
print('use scuttlebutt_attack::comm_trace::{NUM_EVENTS, NUM_CHANNELS, NUM_THREADS};')

print('\n#[no_mangle]')
print('#[link_section = ".rodata.secret"]')
print('pub static CC_SSB_EVENTS: [Event; NUM_EVENTS] = [')
for evt in events:
    print('  Event { thread_id: %d, channel_id: %d, kind: EventKind::%s, '
            'range: %d .. %d, next_event_for_thread: %d },' %
            (evt['thread_id'], evt['channel_id'], evt['kind'],
                evt['range_start'], evt['range_end'],
                evt['next_event_for_thread']))
print('];')

print('\n#[no_mangle]')
print('#[link_section = ".rodata.secret"]')
print('pub static CC_SSB_NUM_VALID_EVENTS: usize = %d;' % num_valid_events)

print('\n#[no_mangle]')
print('#[link_section = ".rodata.secret"]')
print('pub static CC_SSB_CHANNELS: [Channel; NUM_CHANNELS] = [')
for ch in channels:
    print('  Channel { start: %d },' % (ch['start'],))
print('];')

print('\n#[no_mangle]')
print('#[link_section = ".rodata.secret"]')
print('pub static CC_SSB_THREADS: [Thread; NUM_THREADS] = [')
for thr in threads:
    print('  Thread { first_event: %d },' % (thr['first_event'],))
print('];')
