use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;
use crate::backend::DataAppend;
use crate::blocking::CompatAppend;

pub struct AppendCompat {
    inner: Box<dyn DataAppend>,
    rt: Handle
}

impl AppendCompat {
    pub(crate) fn new(rt: Handle, inner: Box<dyn DataAppend>) -> Self {
        Self { rt, inner }
    }
}

impl std::io::Write for AppendCompat {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.rt.block_on(self.inner.write(buf))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.inner.flush())
    }
}

impl CompatAppend for AppendCompat {
    fn close(&mut self) -> std::io::Result<()> {
        self.rt.block_on(self.inner.close())
    }
}