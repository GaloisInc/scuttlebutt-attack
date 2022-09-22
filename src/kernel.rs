use core::slice;
use crate::comm_trace::ThreadState;
use crate::comm_trace_data;
use crate::io_kernel::{SYS_READ, SYS_WRITE, SYS_EXIT};
use crate::io_merged;


pub unsafe fn syscall<const THREAD_ID: usize>(
    num: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
) -> usize {
    match num {
        SYS_READ => read::<THREAD_ID>(arg0, arg1, arg2),
        SYS_WRITE => write::<THREAD_ID>(arg0, arg1, arg2),
        SYS_EXIT => io_merged::exit(),
        _ => panic!("bad syscall: {:?}", (num, arg0, arg1, arg2)),
    }
}

static mut THREAD_STATE: Option<ThreadState> = None;

unsafe fn thread_state<const THREAD_ID: usize>() -> &'static mut ThreadState {
    if THREAD_STATE.is_none() {
        THREAD_STATE = Some(ThreadState::new(
            comm_trace_data::events(),
            &comm_trace_data::threads()[THREAD_ID],
        ));
    }
    THREAD_STATE.as_mut().unwrap()
}

pub unsafe fn read<const THREAD_ID: usize>(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = slice::from_raw_parts_mut(ptr as *mut u8, len);
    let state = thread_state::<THREAD_ID>();
    state.recv(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}

pub unsafe fn write<const THREAD_ID: usize>(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = slice::from_raw_parts(ptr as *const u8, len);
    let state = thread_state::<THREAD_ID>();
    state.send(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}
