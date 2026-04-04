use crate::backend::{
    DataAppend, DataRead, DataVfs, SizedQuery, DataUsage, VfsReader, VfsWriter,
};
use async_trait::async_trait;
use chrono::{DateTime, Local, Utc};
use opendal::layers::{ConcurrentLimitLayer, LoggingLayer, ThrottleLayer};
use opendal::{Metadata, Operator};
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;
use uuid::Uuid;
use crate::fs::FilterOptions;
use crate::operator::{DataInfo, DataInner, DataMode};
use crate::remote::{path_to_str, RemoteConfig};
use crate::remote::query::OpenDALQuery;
use crate::remote::reader::OpenDALReader;
use crate::remote::writer::OpenDALWriter;

/// `OpenDALBackend` contains a wrapper around an [`Operator`] of the `OpenDAL` library.
#[derive(Clone, Debug)]
pub struct OpenDALBackend {
    pub(crate) id: Uuid,
    pub(crate) operator: Operator,
    pub(crate) config: RemoteConfig,
}

impl OpenDALBackend {
    /// Creates a new [`OpenDALBackend`] with the specified config.
    /// 
    /// # Arguments
    /// `config` - The [`RemoteConfig`] to use.
    /// 
    /// # Errors
    /// If the OpenDAL system fails to initialize.
    pub fn new(config: RemoteConfig) -> io::Result<Self> {
        let mut operator = Operator::via_iter(config.src.scheme(), config.src.clone().to_map())
            .map_err(|x| io::Error::from(x))?; // *** map to IO error to not expose opendal.
        if let Some(x) = config.bandwidth {
            operator = operator.layer(ThrottleLayer::new(x.bandwidth, x.burst));
        }
        if let Some(x) = config.max_threads {
            operator = operator.layer(ConcurrentLimitLayer::new(x as usize));
        }
        operator = operator.layer(LoggingLayer::default()); // *** finally, add some logging!
        Ok(Self {
            id: Uuid::new_v4(),
            operator,
            config,
        })
    }

    /// Converts an [`opendal::types::Metadata`] into a valid [`crate::Metadata`] instance.
    /// 
    /// # Arguments
    /// * `path` - The [`Path`] to represent.
    /// * `meta` - The [`Metadata`] to convert.
    /// 
    /// # Returns
    /// A valid [`crate::Metadata`] for use with operations.
    pub(crate) fn meta(path: &Path, meta: &Metadata) -> crate::fs::Metadata {
        crate::fs::Metadata {
            path: path.to_path_buf(),
            is_dir: meta.is_dir(),
            mtime: meta
                .last_modified()
                .map(SystemTime::from)
                .map(DateTime::<Utc>::from),
            atime: None,
            ctime: None,
            size: meta.content_length(),
        }
    }

    /// Converts an [`opendal::types::Metadata`] into a valid [`crate::Metadata`] instance.
    ///
    /// # Arguments
    /// * `path` - The OpenDAL path to represent.
    /// * `meta` - The [`Metadata`] to convert.
    ///
    /// # Returns
    /// A valid [`crate::Metadata`] for use with operations.
    pub(crate) fn meta_str(path: &str, meta: &Metadata) -> io::Result<crate::fs::Metadata> {
        let path =
            PathBuf::from_str(path).map_err(|e| io::Error::new(ErrorKind::InvalidInput, e))?;
        Ok(Self::meta(&path, meta))
    }

    /// Converts an [`opendal::Entry`] into a valid [`crate::Metadata`] instance.
    pub(crate) fn meta_entry(entry: opendal::Entry) -> io::Result<crate::fs::Metadata> {
        Self::meta_str(entry.path(), entry.metadata())
    }
}

impl DataVfs for OpenDALBackend {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn to_inner(self) -> DataInner {
        let id = self.id;
        let ret = Arc::new(self.clone());
        DataInner {
            id,
            info: DataMode::Remote(self.config),
            reader: Some(ret.clone()),
            writer: Some(ret.clone()),
            full: None,
        }
    }
}

#[async_trait]
impl VfsReader for OpenDALBackend {
    fn realpath(&self, item: &Path) -> PathBuf {
        item.to_path_buf() // *** already in full absolute form.
    }

    async fn get_usage(&self) -> Option<io::Result<DataUsage>> {
        None
    }

    async fn open_read(&self, item: &Path) -> io::Result<Box<dyn DataRead>> {
        let ret = OpenDALReader::new(item.to_path_buf(), self.operator.clone()).await?;
        Ok(Box::new(ret))
    }

    async fn get_metadata(&self, item: &Path) -> io::Result<Option<crate::fs::Metadata>> {
        let path = path_to_str(item, false);
        if !self.operator.exists(&path).await? {
            return Ok(None);
        }
        let meta = self.operator.stat(&path).await?;
        let x_meta = Self::meta_str(&path, &meta)?;
        Ok(Some(x_meta))
    }

    async fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQuery>> {
        let path = path_to_str(&item, true);
        let ret = OpenDALQuery::new(self.operator.clone(), path, opts, recursive, include_root)?;
        Ok(Arc::new(ret))
    }
}

#[async_trait]
impl VfsWriter for OpenDALBackend {
    async fn remove_dir(&self, dirname: &Path) -> io::Result<()> {
        self.operator
            .remove_all(&path_to_str(dirname, true))
            .await?;
        Ok(())
    }

    async fn remove_file(&self, filename: &Path) -> io::Result<()> {
        self.operator.delete(&path_to_str(filename, false)).await?;
        Ok(())
    }

    async fn create_dir(&self, item: &Path) -> io::Result<()> {
        self.operator.create_dir(&path_to_str(item, true)).await?;
        Ok(())
    }

    async fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        Ok(())
    }

    async fn set_length(&self, item: &Path, size: u64) -> io::Result<()> {
        if size != 0 {
            Err(ErrorKind::Unsupported.into())
        } else {
            self.operator
                .write(&path_to_str(item, false), Vec::<u8>::new())
                .await?;
            Ok(())
        }
    }

    async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        // Check to see if the current spot is a directory or file.
        let is_dir = self
            .get_metadata(old)
            .await?
            .map(|x| x.is_dir)
            .ok_or(io::Error::from(ErrorKind::NotFound))?;
        let src = path_to_str(old, is_dir);
        let dst = path_to_str(new, is_dir);
        self.operator.rename(&src, &dst).await?;
        Ok(())
    }

    async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        let is_dir = self
            .get_metadata(old)
            .await?
            .map(|x| x.is_dir)
            .ok_or(io::Error::from(ErrorKind::NotFound))?;
        let src = path_to_str(old, is_dir);
        let dst = path_to_str(new, is_dir);
        self.operator.copy(&src, &dst).await?;
        Ok(())
    }

    async fn open_append(&self, item: &Path, truncate: bool) -> io::Result<Box<dyn DataAppend>> {
        let ret = OpenDALWriter::new(item.to_path_buf(), self.operator.clone(), truncate).await?;
        Ok(Box::new(ret))
    }
}
