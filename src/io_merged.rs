use core::cell::RefCell;
use genio::{Read, Write};
use crate::comm_trace::ThreadState;
use crate::comm_trace_data;

#[derive(Clone, Copy)]
pub struct MergedChannel<'a> {
    state: &'a RefCell<ThreadState>,
    channel_id: usize,
}

impl<'a> MergedChannel<'a> {
    pub fn new(
        state: &'a RefCell<ThreadState>,
        channel_id: usize,
    ) -> MergedChannel<'a> {
        MergedChannel { state, channel_id }
    }
}

impl Read for MergedChannel<'_> {
    type ReadError = ();

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let n = self.state.borrow_mut().recv(
            comm_trace_data::events(),
            comm_trace_data::data(),
            self.channel_id,
            buf,
        );
        Ok(n)
    }
}

impl Write for MergedChannel<'_> {
    type WriteError = ();
    type FlushError = ();

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let n = self.state.borrow_mut().send(
            comm_trace_data::events(),
            comm_trace_data::data(),
            self.channel_id,
            buf,
        );
        Ok(n)
    }

    fn flush(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {}
}


#[cfg(not(feature = "microram"))]
pub fn exit() -> ! {
    std::eprintln!("called exit()");
    std::process::exit(0);
}

#[cfg(feature = "microram")]
pub fn exit() -> ! {
    extern "C" {
        fn __cc_exit() -> !;
    }
    unsafe { __cc_exit() };
}
