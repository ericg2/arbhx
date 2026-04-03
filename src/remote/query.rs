use crate::backend::{DataIgnore, MetaStream, SizedQuery};
use crate::util::SimpleIgnore;
use async_trait::async_trait;
use futures_lite::StreamExt;
use opendal::options::ListOptions;
use opendal::{Entry, Operator};
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use crate::fs::{FilterOptions, Metadata};
use crate::remote::data::OpenDALBackend;
use crate::remote::path_to_str;

///
pub struct OpenDALQuery {
    pub(crate) operator: Operator,
    pub(crate) path: String,
    pub(crate) opts: FilterOptions,
    pub(crate) sort: SimpleIgnore,
    pub(crate) recursive: bool,
    pub(crate) root: bool,
}

impl OpenDALQuery {
    pub(crate) fn new(
        operator: Operator,
        path: String,
        opts: Option<FilterOptions>,
        recursive: bool,
        root: bool,
    ) -> io::Result<Self> {
        let opts = opts.unwrap_or_default();
        Ok(Self {
            sort: SimpleIgnore::new(&opts)?,
            operator,
            path,
            opts,
            recursive,
            root,
        })
    }

    fn get_meta(entry: Entry) -> Metadata {
        let path = PathBuf::from(entry.path());
        OpenDALBackend::meta(&path, entry.metadata())
    }

    fn get_entry(&self, res: Result<Entry, opendal::Error>) -> io::Result<Option<Metadata>> {
        let meta = Self::get_meta(res?);
        if !self.root && path_to_str(&meta.path, meta.is_dir) == self.path {
            return Ok(None); // *** 3-28-26: don't include the root unless asking for it.
        }
        match self.sort.filter_ok(&meta)? {
            true => Ok(Some(meta)),
            false => Ok(None),
        }
    }
}

#[async_trait]
impl SizedQuery for OpenDALQuery {
    async fn size(self: Arc<Self>) -> io::Result<Option<u64>> {
        let x = self.operator.stat(&self.path).await?;
        Ok(Some(x.content_length()))
    }

    async fn stream(self: Arc<Self>) -> io::Result<Pin<Box<MetaStream>>> {
        let iter = self
            .operator
            .lister_options(
                &self.path,
                ListOptions {
                    recursive: self.recursive,
                    ..Default::default()
                },
            )
            .await?
            .filter_map(move |x| self.get_entry(x).transpose());
        Ok(Box::pin(iter))
    }
}
