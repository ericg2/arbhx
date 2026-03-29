use futures_lite::StreamExt;
use tokio::runtime::Handle;
use rustic_core::{DataFile, DataIterator, DataStream};

pub struct DataStreamCompat {
    handle: Handle,
    stream: Box<dyn DataStream>,
}

impl DataStreamCompat {
    pub fn new(handle: Handle, stream: Box<dyn DataStream>) -> Self {
        Self { handle, stream }
    }
}

impl Iterator for DataStreamCompat {
    type Item = std::io::Result<DataFile>;

    fn next(&mut self) -> Option<Self::Item> {
        self.handle.block_on(self.stream.next())
    }
}
