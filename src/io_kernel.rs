//! IO module that works by making syscalls to the kernel.
use core::cell::RefCell;
use genio::{Read, Write};
use crate::comm_trace::ThreadState;

extern "C" {
    fn __cc_syscall(num: usize, arg0: usize, arg1: usize, arg2: usize) -> usize;
}

pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_EXIT: usize = 2;


#[derive(Clone, Copy)]
pub struct KernelChannel {
    channel_id: usize,
}

impl KernelChannel {
    pub fn new(
        channel_id: usize,
    ) -> KernelChannel {
        KernelChannel { channel_id }
    }
}

impl Read for KernelChannel {
    type ReadError = ();

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        unsafe {
            let n = __cc_syscall(
                SYS_READ,
                self.channel_id,
                buf.as_mut_ptr() as usize,
                buf.len(),
            );
            Ok(n)
        }
    }
}

impl Write for KernelChannel {
    type WriteError = ();
    type FlushError = ();

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        unsafe {
            let n = __cc_syscall(
                SYS_WRITE,
                self.channel_id,
                buf.as_ptr() as usize,
                buf.len(),
            );
            Ok(n)
        }
    }

    fn flush(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {}
}


pub fn exit() -> ! {
    unsafe {
        __cc_syscall(SYS_EXIT, 0, 0, 0);
    }
    loop {}
}
