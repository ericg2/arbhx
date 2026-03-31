use super::operator::DataInner;
use crate::DataFile;
use crate::backend::SizedQuery;
use futures_lite::{Stream, StreamExt};
use std::io;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct DataQuery {
    query: Arc<dyn SizedQuery>,
    be: Arc<DataInner>,
}

impl DataQuery {
    pub(crate) fn new(be: Arc<DataInner>, query: Arc<dyn SizedQuery>) -> Self {
        Self { be, query }
    }

    pub(crate) fn to_query(self) -> Arc<dyn SizedQuery> {
        self.query
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

pub(crate) type FileStream = dyn Stream<Item = io::Result<DataFile>> + Send;
