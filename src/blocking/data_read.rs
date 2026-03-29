use rustic_core::{AsyncDataRead, DataRead};
use std::io::{Read, Seek, SeekFrom};
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use tokio::runtime::Handle;

pub struct DataReadCompat {
    handle: Handle,
    file: Box<dyn AsyncDataRead>,
}

impl DataReadCompat {
    pub fn new(handle: Handle, file: Box<dyn AsyncDataRead>) -> Self {
        Self { handle, file }
    }
}

impl Read for DataReadCompat {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.handle.block_on(self.file.read(buf))
    }
}

impl Seek for DataReadCompat {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.handle.block_on(self.file.seek(pos))
    }
}

impl DataRead for DataReadCompat {}
