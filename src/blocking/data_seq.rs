use rustic_core::{AsyncSeqWrite, SeqWrite};
use std::io::Write;
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;

pub struct SeqWriteCompat {
    handle: Handle,
    file: Box<dyn AsyncSeqWrite>,
}

impl SeqWriteCompat {
    pub fn new(handle: Handle, file: Box<dyn AsyncSeqWrite>) -> Self {
        Self { handle, file }
    }
}

impl SeqWrite for SeqWriteCompat {
    fn close(&mut self) -> std::io::Result<()> {
        self.handle.block_on(self.file.close())
    }
}

impl Write for SeqWriteCompat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.handle.block_on(self.file.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.handle.block_on(self.file.flush())
    }
}
