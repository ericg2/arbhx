use std::collections::BTreeMap;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::fmt::Debug;
use std::io;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use bytes::Bytes;
use bytesize::ByteSize;
use futures_lite::Stream;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};
use uuid::Uuid;
use crate::fs::{FilterOptions, Metadata};
use crate::operator::DataInner;

#[async_trait]
pub trait DataRead: AsyncRead + AsyncSeek + Send + Sync + 'static + Debug + Unpin  {}

#[async_trait]
pub trait DataAppend: AsyncWrite + Send + Sync + 'static + Debug + Unpin  {
    /// Finalize and close the stream.
    ///
    /// # Errors
    /// Returns an error if the stream cannot be properly finalized.
    async fn close(&mut self) -> io::Result<()>;
}

/// Combined random-access read/write stream.
pub trait DataFull: DataRead + DataAppend {}

/// Read-only virtual filesystem interface.
///
/// Provides metadata access and directory listing functionality.
#[async_trait]
pub trait VfsReader: Send + Sync + 'static + Debug + Unpin {
    /// Convert a relative path into a backend-specific absolute path.
    fn realpath(&self, item: &Path) -> PathBuf;

    /// Retrieves usage information if applicable.
    async fn get_usage(&self) -> Option<io::Result<DataUsage>>;

    /// Reads the path and opens a handle for it.
    async fn open_read(&self, item: &Path) -> io::Result<Box<dyn DataRead>>;

    /// Retrieve metadata for a path.
    ///
    /// # Returns
    /// `Some(metadata)` if the item exists, otherwise `None`.
    ///
    /// # Errors
    /// Returns an error if metadata cannot be retrieved.
    async fn get_metadata(&self, item: &Path) -> io::Result<Option<Metadata>>;

    /// List directory contents.
    ///
    /// # Arguments
    /// * `recursive` - Whether to recurse into subdirectories.
    /// * `root` - Whether to show metadata of the root.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read.
    async fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<Arc<dyn SizedQuery>>;
}

#[async_trait]
pub trait SizedQuery: Send + Sync {
    async fn size(self: Arc<Self>) -> io::Result<Option<u64>>;

    async fn stream(self: Arc<Self>) -> io::Result<Pin<Box<MetaStream>>>;
}

pub type MetaStream = dyn Stream<Item=io::Result<Metadata>> + Send;

pub trait DataVfs {
    /// Retrieves the ID for the [`VfsReader`]. Useful for cross-FS.
    fn get_id(&self) -> Uuid;

    fn to_inner(self) -> DataInner;
}

pub trait VfsConfig: Serialize + DeserializeOwned + Send + Sync + 'static + Debug {
    fn to_backend(self) -> io::Result<Arc<DataInner>>;
}

/// Writable virtual filesystem interface.
///
/// Extends [`VfsReader`] with mutation operations.
#[async_trait]
pub trait VfsWriter: Send + Sync + 'static + Debug + Unpin {
    /// Recursively remove a directory and all contents.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    async fn remove_dir(&self, dirname: &Path) -> io::Result<()>;

    /// Remove a file.
    ///
    /// # Notes
    /// * Must not remove directories.
    /// * Must remove the symlink itself if applicable.
    ///
    /// # Errors
    /// Returns an error if removal fails.
    async fn remove_file(&self, filename: &Path) -> io::Result<()>;

    /// Create a directory and any missing parents.
    ///
    /// # Errors
    /// Returns an error if creation fails.
    async fn create_dir(&self, item: &Path) -> io::Result<()>;

    /// Set file timestamps if the file exists.
    ///
    /// # Errors
    /// Returns an error if timestamps cannot be applied.
    async fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()>;

    /// Set file length.
    ///
    /// # Notes
    /// * Existing files should be resized.
    /// * Missing files should be created.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn set_length(&self, item: &Path, size: u64) -> io::Result<()>;

    /// Move or rename a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn move_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Copy a path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()>;

    /// Opens the specified path in append mode.
    async fn open_append(&self, item: &Path, truncate: bool) -> io::Result<Box<dyn DataAppend>>;
}

#[async_trait]
pub trait VfsFull: VfsReader + VfsWriter + Send + Sync + 'static + Debug + Unpin {
    /// Opens the specified path in random access mode if applicable.
    async fn open_full(&self, item: &Path) -> io::Result<Box<dyn DataFull>>;
}

pub trait DataIgnore {
    /// Checks if a filter is correct.
    fn filter_ok(&self, meta: &Metadata) -> io::Result<bool>;
}

/// Represents the current usage for a VFS.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DataUsage {
    /// The allocated bytes for the store.
    pub max_bytes: ByteSize,
    /// The used bytes for the store.
    pub used_bytes: ByteSize,
    /// The free bytes for the store.
    pub free_bytes: ByteSize
}
