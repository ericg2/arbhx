use crate::backend::SizedQuery;
use futures_lite::{Stream, StreamExt};
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use crate::DataFile;
use super::operator::DataInner;

#[derive(Clone)]
pub struct DataQuery {
    query: Arc<dyn SizedQuery>,
    be: Arc<DataInner>,
}

impl DataQuery {
    pub(crate) fn new(be: Arc<DataInner>, query: Arc<dyn SizedQuery>) -> Self {
        Self { be, query }
    }
    
    async fn size(&self) -> io::Result<Option<u64>> {
        self.query.clone().size().await
    }
    async fn stream(&self) -> io::Result<Pin<Box<FileStream>>> {
        let be = self.be.clone();
        let ret = self
            .query
            .clone()
            .stream()
            .await?
            .map(move |res| res.map(|meta| DataFile::new(meta, be.clone())));
        Ok(Box::pin(ret))
    }
}

pub type FileStream = dyn Stream<Item = io::Result<DataFile>> + Send;
