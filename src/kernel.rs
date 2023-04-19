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
        SYS_EXIT => exit::<THREAD_ID>(),
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

const PERMISSION_BIT: usize = 1 << 31;

fn check_user_buffer(ptr: usize, len: usize) {
    assert!(ptr & PERMISSION_BIT == 0, "buffer must be in userspace");
    let end = ptr.checked_add(len).unwrap();
    assert!(end & PERMISSION_BIT == 0, "buffer must be in userspace");
    // The high bits of `ptr` and `end` must be the same.  This catches the case of a buffer that
    // starts in one unprivileged region, passes through a privileged region, and ends in a
    // different unprivileged region.
    assert!(ptr & !(PERMISSION_BIT - 1) == end & !(PERMISSION_BIT - 1), "buffer is too large");
}

unsafe fn user_buffer<'a>(ptr: usize, len: usize) -> &'a [u8] {
    check_user_buffer(ptr, len);
    slice::from_raw_parts(ptr as *const u8, len)
}

unsafe fn user_buffer_mut<'a>(ptr: usize, len: usize) -> &'a mut [u8] {
    check_user_buffer(ptr, len);
    slice::from_raw_parts_mut(ptr as *mut u8, len)
}

pub unsafe fn read<const THREAD_ID: usize>(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = user_buffer_mut(ptr, len);
    let state = thread_state::<THREAD_ID>();
    state.recv(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}

pub unsafe fn write<const THREAD_ID: usize>(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = user_buffer(ptr, len);
    let state = thread_state::<THREAD_ID>();
    state.send(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}

pub unsafe fn exit<const THREAD_ID: usize>() -> usize {
    let state = thread_state::<THREAD_ID>();
    assert!(state.is_done(comm_trace_data::events()));
    io_merged::exit();
}
