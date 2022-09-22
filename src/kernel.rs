use core::slice;
use crate::comm_trace::ThreadState;
use crate::comm_trace_data;
use crate::io_kernel::{SYS_READ, SYS_WRITE, SYS_EXIT};
use crate::io_merged;


pub unsafe fn syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    match num {
        SYS_READ => read(arg0, arg1, arg2),
        SYS_WRITE => write(arg0, arg1, arg2),
        SYS_EXIT => io_merged::exit(),
        _ => panic!("bad syscall: {:?}", (num, arg0, arg1, arg2)),
    }
}

static mut THREAD_STATE: Option<ThreadState> = None;

pub unsafe fn init_thread_state(thread_id: usize) {
    THREAD_STATE = Some(ThreadState::new(
        comm_trace_data::events(),
        &comm_trace_data::threads()[thread_id],
    ));
}

pub unsafe fn read(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = slice::from_raw_parts_mut(ptr as *mut u8, len);
    let state = THREAD_STATE.as_mut().unwrap();
    state.recv(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}

pub unsafe fn write(channel_id: usize, ptr: usize, len: usize) -> usize {
    let buf = slice::from_raw_parts(ptr as *const u8, len);
    let state = THREAD_STATE.as_mut().unwrap();
    state.send(
        comm_trace_data::events(),
        comm_trace_data::data(),
        channel_id,
        buf,
    )
}

#[cfg(not(feature = "microram"))]
#[no_mangle]
pub unsafe extern "C" fn __cc_syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    syscall(num, arg0, arg1, arg2)
}
