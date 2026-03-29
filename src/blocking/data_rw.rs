use rustic_core::{AsyncDataReadWrite, DataRead, DataReadWrite, SeqWrite};
use std::io::{Read, Seek, SeekFrom, Write};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::runtime::Handle;

pub struct DataReadWriteCompat {
    handle: Handle,
    file: Box<dyn AsyncDataReadWrite>,
}

impl DataReadWriteCompat {
    pub fn new(handle: Handle, file: Box<dyn AsyncDataReadWrite>) -> Self {
        Self { handle, file }
    }
}

impl Read for DataReadWriteCompat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.handle.block_on(self.file.read(buf))
    }
}

impl Seek for DataReadWriteCompat {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.handle.block_on(self.file.seek(pos))
    }
}

impl Write for DataReadWriteCompat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.handle.block_on(self.file.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.handle.block_on(self.file.flush())
    }
}

impl SeqWrite for DataReadWriteCompat {
    fn close(&mut self) -> std::io::Result<()> {
        self.handle.block_on(self.file.close())
    }
}

impl DataRead for DataReadWriteCompat {}

impl DataReadWrite for DataReadWriteCompat {}
