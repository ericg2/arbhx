use crate::backend::{
    DataAppend, DataFull, DataRead, DataVfs, DataUsage, VfsConfig, VfsFull, VfsReader, VfsWriter,
};
use crate::fs::{DataFile, DataQuery, FilterOptions, Metadata};
use crate::local::LocalConfig;
use crate::remote::RemoteConfig;
use bytesize::ByteSize;
use chrono::{DateTime, Local};
use delegate::delegate;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
/// Represents the core [`DataVfs`] based on capabilities. This is designed
/// to wrap an [`Operator`] or [`DataFile`] to reduce repetition.
pub(crate) struct DataInner {
    /// The ID to use for equality comparisons.
    pub id: Uuid,
    /// The info to use for encoding/decoding.
    pub info: DataMode,
    /// The [`VfsReader`] system if applicable.
    pub reader: Option<Arc<dyn VfsReader>>,
    /// The [`VfsWriter`] system if applicable.
    pub writer: Option<Arc<dyn VfsWriter>>,
    /// The [`VfsFull`] system if applicable.
    pub full: Option<Arc<dyn VfsFull>>,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug, Hash)]
#[non_exhaustive]
pub enum DataMode {
    Local(LocalConfig),
    Remote(RemoteConfig),
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct DataInfo {
    pub id: Uuid,
    pub mode: DataMode,
    pub can_read: bool,
    pub can_append: bool,
    pub can_full: bool,
}

impl PartialEq<Self> for DataInner {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DataInner {}

impl DataInner {
    /// # Returns
    /// A [`VfsReader`] instance, or [`ErrorKind::Unsupported`].
    fn reader(&self) -> io::Result<&dyn VfsReader> {
        let ret = self
            .reader
            .as_ref()
            .ok_or_else(|| io::Error::from(ErrorKind::Unsupported))?
            .deref();
        Ok(ret)
    }
    /// # Returns
    /// A [`VfsWriter`] instance, or [`ErrorKind::Unsupported`].
    fn writer(&self) -> io::Result<&dyn VfsWriter> {
        let ret = self
            .writer
            .as_ref()
            .ok_or_else(|| io::Error::from(ErrorKind::ReadOnlyFilesystem))?
            .deref();
        Ok(ret)
    }
    /// # Returns
    /// A [`VfsFull`] instance, or [`ErrorKind::Unsupported`].
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
    pub async fn usage(&self) -> io::Result<Option<DataUsage>> {
        self.reader()?.get_usage().await.transpose()
    }
    pub async fn open_read(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn DataRead>> {
        self.reader()?.open_read(item.as_ref()).await
    }
    pub async fn open_full(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn DataFull>> {
        self.full()?.open_full(item.as_ref()).await
    }
    pub async fn stat(&self, item: impl AsRef<Path>) -> io::Result<Option<Metadata>> {
        self.reader()?.get_metadata(item.as_ref()).await
    }
    pub async fn remove_dir(&self, dirname: impl AsRef<Path>) -> io::Result<()> {
        self.writer()?.remove_dir(dirname.as_ref()).await
    }
    pub async fn remove_file(&self, filename: impl AsRef<Path>) -> io::Result<()> {
        self.writer()?.remove_file(filename.as_ref()).await
    }
    pub async fn create_dir(&self, item: impl AsRef<Path>) -> io::Result<()> {
        self.writer()?.create_dir(item.as_ref()).await
    }
    pub async fn set_length(&self, item: impl AsRef<Path>, size: u64) -> io::Result<()> {
        self.writer()?.set_length(item.as_ref(), size).await
    }
    pub async fn move_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()> {
        self.writer()?.move_to(old.as_ref(), new.as_ref()).await
    }
    pub async fn copy_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()> {
        self.writer()?.copy_to(old.as_ref(), new.as_ref()).await
    }

    pub async fn set_times(
        &self,
        item: impl AsRef<Path>,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        self.writer()?.set_times(item.as_ref(), mtime, atime).await
    }

    pub async fn open_append(
        &self,
        item: impl AsRef<Path>,
        truncate: bool,
    ) -> io::Result<Box<dyn DataAppend>> {
        self.writer()?.open_append(item.as_ref(), truncate).await
    }
}

#[derive(Clone, Debug)]
pub struct Operator {
    pub(crate) be: Arc<DataInner>,
}

impl Operator {
    /// Creates a new [`Operator`] with
    pub fn with_info(info: DataMode) -> std::io::Result<Self> {
        match info {
            DataMode::Local(x) => Self::new(x),
            DataMode::Remote(x) => Self::new(x),
        }
    }
    pub fn new<V: VfsConfig>(config: V) -> std::io::Result<Self> {
        let be = config.to_backend()?;
        Ok(Self { be })
    }
    pub fn id(&self) -> Uuid {
        self.be.id
    }
    pub fn info(&self) -> DataInfo {
        DataInfo {
            id: self.be.id,
            mode: self.be.info.clone(),
            can_read: self.be.reader.is_some(),
            can_append: self.be.writer.is_some(),
            can_full: self.be.full.is_some(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum FileAction {
    /// The [`DataFile`] did not exist on the system.
    Created(DataFile),
    /// The [`DataFile`] existed on the system.
    Found(DataFile),
}

impl Deref for FileAction {
    type Target = DataFile;

    fn deref(&self) -> &Self::Target {
        match self {
            FileAction::Created(x) => x,
            FileAction::Found(x) => x,
        }
    }
}

impl FileAction {
    /// Retrieves the [`DataFile`]
    pub fn into_file(self) -> DataFile {
        match self {
            FileAction::Created(x) => x,
            FileAction::Found(x) => x,
        }
    }
}

impl Operator {
    pub async fn list(
        &self,
        item: impl AsRef<Path>,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<DataQuery> {
        let query = self
            .be
            .reader()?
            .read_dir(item.as_ref(), opts, recursive, include_root)
            .await?;
        Ok(DataQuery::new(self.be.clone(), query))
    }

    /// Returns an existing [`DataFile`], or an [`ErrorKind::NotFound`] if not detected.
    pub async fn get_existing(&self, item: impl AsRef<Path>) -> io::Result<DataFile> {
        let meta = self.stat(item).await?.ok_or(io::ErrorKind::NotFound)?;
        let ret = DataFile::new(meta, self.be.clone());
        Ok(ret)
    }

    /// Attempts to retrieve a [`DataFile`], creating if not existing.
    pub async fn ensure_file(&self, item: impl AsRef<Path>) -> io::Result<FileAction> {
        let path = item.as_ref();
        match self.stat(path).await? {
            None => {
                self.set_length(path, 0).await?;
                let file = self.get_existing(path).await?;
                Ok(FileAction::Created(file))
            }
            Some(x) => {
                let file = self.get_existing(path).await?;
                Ok(FileAction::Found(file))
            }
        }
    }

    delegate! {
        to self.be {
            pub async fn stat(&self, item: impl AsRef<Path>) -> io::Result<Option<Metadata>>;
            pub async fn open_read(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn DataRead>>;
            pub async fn open_full(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn DataFull>>;
            pub async fn remove_dir(&self, dirname: impl AsRef<Path>) -> io::Result<()>;
            pub async fn remove_file(&self, filename: impl AsRef<Path>) -> io::Result<()>;
            pub async fn create_dir(&self, item: impl AsRef<Path>) -> io::Result<()>;
            pub async fn set_length(&self, item: impl AsRef<Path>, size: u64) -> io::Result<()>;
            pub async fn move_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()>;
            pub async fn copy_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()>;
            pub async fn usage(&self) -> io::Result<Option<DataUsage >>;
            pub async fn open_append(
                &self,
                item: impl AsRef<Path>,
                truncate: bool,
            ) -> io::Result<Box<dyn DataAppend>>;
            pub async fn set_times(
                &self,
                item: impl AsRef<Path>,
                mtime: DateTime<Local>,
                atime: DateTime<Local>,
            ) -> io::Result<()>;
        }
    }
}
