use crate::backend::DataFull;
use crate::blocking::{CompatAppend, CompatFull, CompatRead};
use std::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::runtime::Handle;

pub struct FullCompat {
    inner: Box<dyn DataFull>,
    rt: Handle,
}

impl FullCompat {
    pub(crate) fn new(rt: Handle, inner: Box<dyn DataFull>) -> Self {
        Self { rt, inner }
    }
}

impl std::io::Write for FullCompat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.inner.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.inner.flush())
    }
}

impl std::io::Read for FullCompat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.inner.read(buf))
    }
}

impl std::io::Seek for FullCompat {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.rt.block_on(self.inner.seek(pos))
    }
}

impl CompatAppend for FullCompat {
    fn close(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.inner.close())
    }
}

impl CompatRead for FullCompat {}

impl CompatFull for FullCompat {}
