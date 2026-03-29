use crate::backend::DataIgnore;
use crate::file::DataFile;
use crate::filters::FilterOptions;
use crate::meta::ExtMetadata;
use crate::opendal::data::OpenDALBackend;
use crate::opendal::handle::OpenDALHandle;
use crate::opendal::path_to_str;
use crate::query::{DataQuery, DataStream};
use crate::sort::SimpleIgnore;
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use futures_lite::StreamExt;
use opendal::options::ListOptions;
use opendal::{Entry, Operator};
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

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

    fn get_file(operator: &Operator, entry: Entry) -> DataFile {
        let path = PathBuf::from(entry.path());
        let meta = OpenDALBackend::meta(&path, entry.metadata());
        let reader = OpenDALHandle::new(operator.clone(), &path, meta.is_dir);
        DataFile::new(meta, Arc::new(reader), true)
    }

    fn get_entry(&self, res: Result<Entry, opendal::Error>) -> io::Result<Option<DataFile>> {
        let file = Self::get_file(&self.operator, res?);
        if !self.root && path_to_str(&file.meta.path, file.meta.is_dir) == self.path {
            return Ok(None); // *** 3-28-26: don't include the root unless asking for it.
        }
        match self.sort.filter_ok(&file.meta)? {
            true => Ok(Some(file)),
            false => Ok(None),
        }
    }
}

#[async_trait]
impl DataQuery for OpenDALQuery {
    async fn size(self: Arc<Self>) -> io::Result<Option<u64>> {
        let x = self.operator.stat(&self.path).await?;
        Ok(Some(x.content_length()))
    }

    async fn options(&self) -> FilterOptions {
        self.opts.clone()
    }

    async fn get_iter(self: Arc<Self>) -> io::Result<Pin<Box<dyn DataStream>>> {
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

    async fn path(&self) -> &Path {
        todo!()
    }
}
