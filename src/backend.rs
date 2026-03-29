use crate::file::DataFile;
use crate::filters::FilterOptions;
use crate::meta::ExtMetadata;
use crate::query::DataQuery;
use async_trait::async_trait;
use bytes::Bytes;
use chrono::{DateTime, Local};
use std::error::Error;
use std::ffi::OsStr;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read, Seek, Write};
use std::num::TryFromIntError;
use std::ops::DerefMut;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};
use tokio::runtime::Handle;
use uuid::Uuid;

#[async_trait]
pub trait FileOpener: Send + Sync + 'static {
    /// Returns `true` if the handle supports read access.
    fn can_read(&self) -> bool;

    /// Returns `true` if the handle supports sequential append writes.
    fn can_append(&self) -> bool;

    /// Returns `true` if the handle supports full random read/write access.
    fn can_full(&self) -> bool;

    /// Open a read-only stream.
    ///
    /// # Errors
    /// Returns an error if reading is not supported or the file cannot be opened.
    async fn open_read(&self) -> io::Result<Box<dyn DataRead>>;

    /// Open a sequential write stream.
    ///
    /// # Arguments
    /// * `truncate` - If `true`, existing contents are discarded.
    ///
    /// # Errors
    /// Returns an error if writing is not supported or the file cannot be opened.
    async fn open_append(&self, truncate: bool) -> io::Result<Box<dyn DataAppend>>;

    /// Open a random-access read/write stream.
    ///
    /// # Errors
    /// Returns an error if full access is not supported or the file cannot be opened.
    async fn open_full(&self) -> io::Result<Box<dyn DataFull>>;

    /// List directory contents NOT recursive.
    ///
    /// # Errors
    /// Returns an error if the directory cannot be read.
    async fn read_dir(&self) -> io::Result<Arc<dyn DataQuery>>;

    /// Delete the file or directory.
    ///
    /// # Errors
    /// Returns an error if deletion fails or is not permitted.
    async fn delete(&self) -> io::Result<()>;

    /// Rename or move the item.
    ///
    /// # Arguments
    /// * `dest` - Destination path.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    async fn rename(&self, dest: &Path) -> io::Result<()>;
}

#[async_trait]
pub trait DataRead: AsyncRead + AsyncSeek + Send + Sync + Unpin + 'static {
    // Returns the underlying path of this stream.
    //fn path(&self) -> PathBuf;
}

#[async_trait]
pub trait DataAppend: AsyncWrite + Send + Sync + Unpin + 'static {
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
    /// Retrieves the ID for the [`VfsReader`]. Useful for cross-FS.
    fn get_id(&self) -> Uuid;

    /// Retrieves usage information if applicable.
    async fn get_usage(&self) -> Option<io::Result<UsageStat>>;

    /// Retrieve metadata for a path.
    ///
    /// # Returns
    /// `Some(metadata)` if the item exists, otherwise `None`.
    ///
    /// # Errors
    /// Returns an error if metadata cannot be retrieved.
    async fn get_metadata(&self, item: &Path) -> io::Result<Option<ExtMetadata>>;

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
    ) -> io::Result<Arc<dyn DataQuery>>;

    /// Retrieve a file only if its size matches.
    ///
    /// # Returns
    /// `Some(DataFile)` if the file exists and matches the size.
    ///
    /// # Errors
    /// Returns an error if metadata retrieval fails.
    async fn get_matching(&self, item: &Path, size: u64) -> io::Result<Option<DataFile>> {
        match self.get_metadata(item).await? {
            Some(meta) if !meta.is_dir && meta.size == size => self.get_existing(item).await,
            _ => Ok(None),
        }
    }

    /// Convert a relative path into a backend-specific absolute path.
    async fn realpath(&self, item: &Path) -> PathBuf;

    /// Retrieve an existing file entry.
    ///
    /// # Returns
    /// `Some(DataFile)` if the item exists, otherwise `None`.
    ///
    /// # Errors
    /// Returns an error if the lookup fails.
    async fn get_existing(&self, item: &Path) -> io::Result<Option<DataFile>>;
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
}

#[async_trait]
pub trait VfsBackend: VfsReader + VfsWriter + Send + Sync + 'static + Debug + Unpin {}

impl dyn VfsBackend {
    async fn ensure_file(&self, item: &Path) -> io::Result<DataFile> {
        match self.get_existing(item).await? {
            Some(x) => Ok(x),
            None => {
                self.set_length(item, 0).await?;
                self.get_existing(item)
                    .await?
                    .ok_or(ErrorKind::NotFound.into())
            }
        }
    }
}

pub trait DataIgnore {
    /// Checks if a filter is correct.
    fn filter_ok(&self, meta: &ExtMetadata) -> io::Result<bool>;
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct UsageStat {
    pub used_bytes: u64,
    pub max_bytes: u64,
}
