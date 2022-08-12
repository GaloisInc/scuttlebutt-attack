use std::cmp;
use std::fs::File;
use std::io;
use std::mem;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use genio::{Read, Write};
use rand::SeedableRng;
use rand_chacha;
use scuttlebutt_attack::{server, attacker};
use serde::Serialize;
use serde_bytes::Bytes;
use serde_cbor;


#[derive(Debug, Serialize)]
pub enum EventKind {
    Read(usize),
    Write(#[serde(with = "serde_bytes")] Box<[u8]>),
}

#[derive(Debug, Serialize)]
pub struct Event {
    channel_id: u32,
    kind: EventKind,
}

#[derive(Debug, Default)]
struct Context {
    events: Vec<Event>,
    next_channel_id: u32,
}

impl Context {
    fn next_channel_id(&mut self) -> u32 {
        let x = self.next_channel_id;
        self.next_channel_id += 1;
        x
    }
}

#[derive(Clone, Default)]
struct ContextRef(Arc<Mutex<Context>>);

impl ContextRef {
    fn emit_read(&self, channel_id: u32, len: usize) {
        let evt = Event {
            channel_id,
            kind: EventKind::Read(len),
        };
        self.0.lock().unwrap().events.push(evt);
    }

    fn emit_write(&self, channel_id: u32, buf: &[u8]) {
        let evt = Event {
            channel_id,
            kind: EventKind::Write(buf.to_owned().into_boxed_slice()),
        };
        self.0.lock().unwrap().events.push(evt);
    }

    pub fn make_channel(&self) -> (ChannelWriter, ChannelReader) {
        let channel_id = self.0.lock().unwrap().next_channel_id();
        let (send, recv) = mpsc::channel();
        let w = ChannelWriter {
            ctx: self.clone(),
            channel_id,
            send,
        };
        let r = ChannelReader {
            ctx: self.clone(),
            channel_id,
            recv,
            buf: Vec::new(),
            consumed: 0,
        };
        (w, r)
    }
}


struct ChannelWriter {
    ctx: ContextRef,
    channel_id: u32,
    send: Sender<Vec<u8>>,
}

struct ChannelReader {
    ctx: ContextRef,
    channel_id: u32,
    recv: Receiver<Vec<u8>>,
    buf: Vec<u8>,
    consumed: usize,
}

impl Write for ChannelWriter {
    type WriteError = ();
    type FlushError = ();

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        self.ctx.emit_write(self.channel_id, buf);
        if buf.len() > 0 {
            let _ = self.send.send(buf.to_owned());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn size_hint(&mut self, _bytes: usize) {}
}

impl Read for ChannelReader {
    type ReadError = ();

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        if self.consumed == self.buf.len() {
            match self.recv.recv() {
                Ok(x) => {
                    assert!(x.len() > 0);
                    self.buf = x;
                    self.consumed = 0;
                },
                Err(_) => {
                    self.ctx.emit_read(self.channel_id, 0);
                    return Ok(0);
                },
            }
        }

        let n = cmp::min(buf.len(), self.buf.len() - self.consumed);
        buf[..n].copy_from_slice(&self.buf[self.consumed .. self.consumed + n]);
        self.consumed += n;
        self.ctx.emit_read(self.channel_id, n);
        return Ok(n);
    }
}


struct ChannelPair(ChannelWriter, ChannelReader);

impl Write for ChannelPair {
    type WriteError = <ChannelWriter as Write>::WriteError;
    type FlushError = <ChannelWriter as Write>::FlushError;

    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::WriteError> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<(), Self::FlushError> {
        self.0.flush()
    }

    fn size_hint(&mut self, bytes: usize) {
        self.0.size_hint(bytes)
    }
}

impl Read for ChannelPair {
    type ReadError = <ChannelReader as Read>::ReadError;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::ReadError> {
        self.1.read(buf)
    }
}


fn main() -> Result<(), serde_cbor::Error> {
    // TODO: read rng seed from command line
    let mut rng = rand_chacha::ChaCha12Rng::from_seed([77; 32]);

    let ctx = ContextRef::default();
    let (w1, r1) = ctx.make_channel();
    let (w2, r2) = ctx.make_channel();
    let channel1 = ChannelPair(w1, r2);
    let channel2 = ChannelPair(w2, r1);

    let s = thread::spawn(move || {
        server::run(&mut rng, channel1).unwrap();
        eprintln!("server: handshake succeeded");
    });
    //let c = thread::spawn(move || client_thread(channel2).unwrap());
    let c = thread::spawn(move || attacker::run(channel2).unwrap());

    s.join().unwrap();
    c.join().unwrap();

    const RECORDING_PATH: &str = "recording.cbor";
    let ctx_state = ctx.0.lock().unwrap();
    let f = File::create(RECORDING_PATH)?;
    serde_cbor::to_writer(f, &ctx_state.events)?;
    eprintln!("wrote {} events to {}", ctx_state.events.len(), RECORDING_PATH);
    Ok(())
}
