#![cfg_attr(feature = "microram", no_std)]
#![cfg_attr(feature = "microram", no_main)]
use core::cell::RefCell;
use rand::SeedableRng;
use scuttlebutt_attack::comm_trace::ThreadState;
use scuttlebutt_attack::comm_trace_data;
use scuttlebutt_attack::io_merged::{self, MergedChannel};
use scuttlebutt_attack::server;
use scuttlebutt_attack::util::ChannelPair;
use ssb_handshake::error::HandshakeError;

extern "C" {
    fn __cc_trace(msg: *const u8);
    fn __cc_trace_exec(
        name: *const u8,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    );
}

macro_rules! cc_trace {
    ($msg:expr) => {
        unsafe { __cc_trace(concat!($msg, "\0").as_ptr()) }
    };
    /*
    ($msg:expr, $($args:tt)*) => {
        unsafe { __cc_trace(format!(concat!($msg, "\0"), $($args)*).as_ptr()) }
    };
    */
}


#[cfg_attr(feature = "microram", no_mangle)]
pub fn main() {
    let mut rng = rand_chacha::ChaCha12Rng::from_seed(scuttlebutt_attack_constants::SEED);

    let state = RefCell::new(ThreadState::new(
        comm_trace_data::events(),
        &comm_trace_data::threads()[0],
    ));

    let to_client = MergedChannel::new(&state, 0);
    let from_client = MergedChannel::new(&state, 1);
    let channel = ChannelPair(to_client, from_client);
    let r = server::run(&mut rng, channel);
    match r {
        Ok(()) => cc_trace!("result: OK"),
        Err(HandshakeError::Io(_)) => cc_trace!("result: ERR (Io)"),
        Err(HandshakeError::UnexpectedEnd) => cc_trace!("result: ERR (UnexpectedEnd)"),
        Err(HandshakeError::ClientHelloDeserializeFailed) => cc_trace!("result: ERR (ClientHelloDeserializeFailed)"),
        Err(HandshakeError::ClientHelloVerifyFailed) => cc_trace!("result: ERR (ClientHelloVerifyFailed)"),
        Err(HandshakeError::ServerHelloDeserializeFailed) => cc_trace!("result: ERR (ServerHelloDeserializeFailed)"),
        Err(HandshakeError::ServerHelloVerifyFailed) => cc_trace!("result: ERR (ServerHelloVerifyFailed)"),
        Err(HandshakeError::ClientAuthDeserializeFailed) => cc_trace!("result: ERR (ClientAuthDeserializeFailed)"),
        Err(HandshakeError::ClientAuthVerifyFailed) => cc_trace!("result: ERR (ClientAuthVerifyFailed)"),
        Err(HandshakeError::ServerAcceptDeserializeFailed) => cc_trace!("result: ERR (ServerAcceptDeserializeFailed)"),
        Err(HandshakeError::ServerAcceptVerifyFailed) => cc_trace!("result: ERR (ServerAcceptVerifyFailed)"),
        Err(HandshakeError::SharedAInvalid) => cc_trace!("result: ERR (SharedAInvalid)"),
        Err(HandshakeError::SharedBInvalid) => cc_trace!("result: ERR (SharedBInvalid)"),
        Err(HandshakeError::SharedCInvalid) => cc_trace!("result: ERR (SharedCInvalid)"),
    }
    r.unwrap();
    io_merged::exit();
}


#[cfg(not(feature = "microram"))]
mod cc {
    use std::ffi::CStr;

    #[no_mangle]
    unsafe extern "C" fn __cc_trace(s: *const u8) {
        eprintln!("[TRACE] {:?}", CStr::from_ptr(s as *const i8));
    }

    #[no_mangle]
    unsafe extern "C" fn __cc_trace_exec(s: *const u8, arg0: usize, arg1: usize, arg2: usize, arg3: usize) {
        eprintln!("[TRACE] {:?}({:x}, {:x}, {:x}, {:x})", CStr::from_ptr(s as *const i8), arg0, arg1, arg2, arg3);
    }
}
