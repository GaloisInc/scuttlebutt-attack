use genio::{Read, Write};


pub struct ChannelPair<W, R>(pub W, pub R);

impl<W: Write, R> Write for ChannelPair<W, R> {
    type WriteError = <W as Write>::WriteError;
    type FlushError = <W as Write>::FlushError;

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

impl<W, R: Read> Read for ChannelPair<W, R> {
    type ReadError = <R as Read>::ReadError;

    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::ReadError> {
        self.1.read(buf)
    }
}

