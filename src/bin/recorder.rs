use std::cmp;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;
use genio::{Read, Write};
use rand::SeedableRng;
use rand_chacha;
use scuttlebutt_attack::{server, attacker};


struct ChannelIo {
    send: Sender<Vec<u8>>,
    recv: Receiver<Vec<u8>>,
    buf: Vec<u8>,
    consumed: usize,
}

impl ChannelIo {
    pub fn new(send: Sender<Vec<u8>>, recv: Receiver<Vec<u8>>) -> ChannelIo {
        ChannelIo {
            send,
            recv,
            buf: Vec::new(),
            consumed: 0,
        }
    }
}

impl Read for ChannelIo {
    type ReadError = ();

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        if self.consumed == self.buf.len() {
            match self.recv.recv() {
                Ok(x) => {
                    assert!(x.len() > 0);
                    self.buf = x;
                    self.consumed = 0;
                },
                Err(_) => return Ok(0),
            }
        }

        let n = cmp::min(buf.len(), self.buf.len() - self.consumed);
        buf[..n].copy_from_slice(&self.buf[self.consumed .. self.consumed + n]);
        self.consumed += n;
        return Ok(n);
    }
}

impl Write for ChannelIo {
    type WriteError = ();
    type FlushError = ();

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
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

fn main() {
    // TODO: read rng seed from command line
    let mut rng = rand_chacha::ChaCha12Rng::from_seed([77; 32]);

    let (send1, recv1) = mpsc::channel();
    let (send2, recv2) = mpsc::channel();
    let channel1 = ChannelIo::new(send1, recv2);
    let channel2 = ChannelIo::new(send2, recv1);
    // TODO: record reads/writes that occur on each channel

    let s = thread::spawn(move || {
        server::run(&mut rng, channel1).unwrap();
        eprintln!("server: handshake succeeded");
    });
    //let c = thread::spawn(move || client_thread(channel2).unwrap());
    let c = thread::spawn(move || attacker::run(channel2).unwrap());

    s.join().unwrap();
    c.join().unwrap();
}
