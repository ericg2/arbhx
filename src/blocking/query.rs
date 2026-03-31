use crate::backend::{MetaStream, SizedQuery};
use crate::blocking::{FileIterator};
use crate::vfs::{DataInner, FileStream};
use crate::{DataFile, ExtMetadata};
use futures_lite::{Stream, StreamExt};
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(Clone)]
pub struct DataQuery {
    rt: Handle,
    query: Arc<dyn SizedQuery>,
    be: Arc<DataInner>,
}

impl DataQuery {
    pub(crate) fn new(rt: Handle, be: Arc<DataInner>, query: Arc<dyn SizedQuery>) -> Self {
        Self { rt, be, query }
    }

    fn size(&self) -> io::Result<Option<u64>> {
        self.rt.block_on(self.query.clone().size())
    }
    fn stream(&self) -> io::Result<Box<FileIterator>> {
        let be = self.be.clone();
        let stream = self.rt.block_on(self.query.clone().stream())?;
        let compat = StreamCompat::new(self.rt.clone(), be, stream);
        Ok(Box::new(compat))
    }
}

struct StreamCompat {
    rt: Handle,
    be: Arc<DataInner>,
    stream: Pin<Box<MetaStream>>,
}

impl StreamCompat {
    pub(crate) fn new(rt: Handle, be: Arc<DataInner>, stream: Pin<Box<MetaStream>>) -> Self {
        Self { rt, be, stream }
    }
}

impl Iterator for StreamCompat {
    type Item = io::Result<DataFile>;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self
            .rt
            .block_on(self.stream.next())?
            .map(|x| DataFile::new(x, self.be.clone()));
        Some(item)
    }
}
