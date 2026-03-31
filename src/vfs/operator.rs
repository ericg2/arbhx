use super::FilterOptions;
use crate::backend::{DataAppend, DataFull, DataRead, DataVfs, UsageStat, VfsFull, VfsReader, VfsWriter};
use crate::local::config::LocalConfig;
use crate::local::data::LocalBackend;

use crate::opendal::config::RemoteConfig;
use crate::opendal::data::OpenDALBackend;
use crate::ExtMetadata;
use crate::{DataConfig, DataQuery};
use chrono::{DateTime, Local};
use delegate::delegate;
use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug)]
pub(crate) struct DataInner {
    pub reader: Option<Arc<dyn VfsReader>>,
    pub writer: Option<Arc<dyn VfsWriter>>,
    pub full: Option<Arc<dyn VfsFull>>,
}

impl DataInner {
    fn reader(&self) -> io::Result<&dyn VfsReader> {
        let ret = self
            .reader
            .as_ref()
            .ok_or_else(|| io::Error::from(ErrorKind::Unsupported))?
            .deref();
        Ok(ret)
    }
    fn writer(&self) -> io::Result<&dyn VfsWriter> {
        let ret = self
            .writer
            .as_ref()
            .ok_or_else(|| io::Error::from(ErrorKind::Unsupported))?
            .deref();
        Ok(ret)
    }
    fn full(&self) -> io::Result<&dyn VfsFull> {
        let ret = self
            .full
            .as_ref()
            .ok_or_else(|| io::Error::from(ErrorKind::Unsupported))?
            .deref();
        Ok(ret)
    }
}

impl DataInner {
    pub async fn usage(&self) -> io::Result<Option<UsageStat>> {
        self.reader()?.get_usage().await.transpose()
    }
    pub async fn open_read(&self, item: &Path) -> io::Result<Box<dyn DataRead>> {
        self.reader()?.open_read(item).await
    }
    pub async fn open_full(&self, item: &Path) -> io::Result<Box<dyn DataFull>> {
        self.full()?.open_full(item).await
    }
    pub async fn stat(&self, item: &Path) -> io::Result<Option<ExtMetadata>> {
        self.reader()?.get_metadata(item).await
    }
    pub async fn remove_dir(&self, dirname: &Path) -> io::Result<()> {
        self.writer()?.remove_dir(dirname).await
    }
    pub async fn remove_file(&self, filename: &Path) -> io::Result<()> {
        self.writer()?.remove_file(filename).await
    }
    pub async fn create_dir(&self, item: &Path) -> io::Result<()> {
        self.writer()?.create_dir(item).await
    }
    pub async fn set_length(&self, item: &Path, size: u64) -> io::Result<()> {
        self.writer()?.set_length(item, size).await
    }
    pub async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.writer()?.move_to(old, new).await
    }
    pub async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.writer()?.copy_to(old, new).await
    }

    pub async fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        self.writer()?.set_times(item, mtime, atime).await
    }

    pub async fn open_append(
        &self,
        item: &Path,
        truncate: bool,
    ) -> io::Result<Box<dyn DataAppend>> {
        self.writer()?.open_append(item, truncate).await
    }
}

#[derive(Clone, Debug)]
pub struct DataOperator {
    pub(crate) be: Arc<DataInner>,
}

impl DataOperator {
    pub fn local(config: LocalConfig) -> std::io::Result<Self> {
        Self::new(DataConfig::Local(config))
    }
    pub fn remote(config: RemoteConfig) -> std::io::Result<Self> {
        Self::new(DataConfig::Remote(config))
    }
    pub fn new(config: DataConfig) -> std::io::Result<Self> {
        let be = Arc::new(match config {
            DataConfig::Local(x) => LocalBackend::new(x)?.to_inner(),
            DataConfig::Remote(x) => OpenDALBackend::new(x)?.to_inner(),
        });
        Ok(Self { be })
    }
}

impl DataOperator {
    pub async fn list(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<DataQuery> {
        let query = self
            .be
            .reader()?
            .read_dir(item, opts, recursive, include_root)
            .await?;
        Ok(DataQuery::new(self.be.clone(), query))
    }
}

impl DataOperator {
    delegate! {
        to self.be {
            pub async fn stat(&self, item: &Path) -> io::Result<Option<ExtMetadata>>;
            pub async fn open_read(&self, item: &Path) -> io::Result<Box<dyn DataRead>>;
            pub async fn open_full(&self, item: &Path) -> io::Result<Box<dyn DataFull>>;
            pub async fn remove_dir(&self, dirname: &Path) -> io::Result<()>;
            pub async fn remove_file(&self, filename: &Path) -> io::Result<()>;
            pub async fn create_dir(&self, item: &Path) -> io::Result<()>;
            pub async fn set_length(&self, item: &Path, size: u64) -> io::Result<()>;
            pub async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;
            pub async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;
            pub async fn usage(&self) -> io::Result<Option<UsageStat>>;
            pub async fn open_append(
                &self,
                item: &Path,
                truncate: bool,
            ) -> io::Result<Box<dyn DataAppend>>;
            pub async fn set_times(
                &self,
                item: &Path,
                mtime: DateTime<Local>,
                atime: DateTime<Local>,
            ) -> io::Result<()>;
        }
    }
}
