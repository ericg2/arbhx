use crate::backend::DataRead;
use crate::blocking::CompatRead;
use tokio::io::AsyncReadExt;
use tokio::io::{AsyncSeekExt, SeekFrom};
use tokio::runtime::Handle;

pub struct ReadCompat {
    inner: Box<dyn DataRead>,
    rt: Handle,
}

impl ReadCompat {
    pub(crate) fn new(rt: Handle, inner: Box<dyn DataRead>) -> Self {
        Self { rt, inner }
    }
}

impl std::io::Read for ReadCompat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.inner.read(buf))
    }
}

impl std::io::Seek for ReadCompat {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.rt.block_on(self.inner.seek(pos))
    }
}

impl CompatRead for ReadCompat {}
