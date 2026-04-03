use super::operator::DataInner;
use crate::backend::{DataAppend, DataFull, DataRead};
use bytesize::ByteSize;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io;
use std::io::{Seek, Write};
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

/// Represents Metadata for a returned file. This is a 
/// data-only `struct`, with no operations as a result.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
    pub(crate) mtime: Option<DateTime<Utc>>,
    pub(crate) atime: Option<DateTime<Utc>>,
    pub(crate) ctime: Option<DateTime<Utc>>,
    pub(crate) size: u64,
}

impl Metadata {
    /// # Returns
    /// The [`Path::file_name`] of this file.
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_default()
    }

    /// # Returns
    /// The full, absolute [`Path`] of the node.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// # Returns
    /// If the [`Path`] is a directory.
    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    /// # Returns
    /// If the [`Path`] is a file (not a directory).
    ///
    /// # Important
    /// This backend does <b>NOT support symbolic links</b>. As a result, this
    /// function is simply the inverse of [`is_dir`] - with no special processing.
    pub fn is_file(&self) -> bool {
        !self.is_dir
    }

    /// # Returns
    /// The last modified [`DateTime`] if supported.
    pub fn mtime(&self) -> Option<DateTime<Utc>> {
        self.mtime.clone()
    }

    /// # Returns
    /// The last accessed [`DateTime`] if supported.
    pub fn atime(&self) -> Option<DateTime<Utc>> {
        self.atime.clone()
    }

    /// # Returns
    /// The [`ByteSize`] of this node.
    pub fn size(&self) -> ByteSize {
        ByteSize(self.size)
    }

    /// Converts [`std::fs::Metadata`] into a valid [`Metadata`] struct.
    ///
    /// # Arguments
    /// * `path` - The [`Path`] to represent.
    /// * `meta` - The [`std::fs::Metadata`] to convert.
    ///
    /// # Returns
    /// A valid [`Metadata`] struct for the conversion.
    pub(crate) fn from_io(path: impl AsRef<Path>, meta: std::fs::Metadata) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            is_dir: meta.is_dir(),
            mtime: meta.modified().ok().map(|x| x.into()),
            atime: meta.accessed().ok().map(|x| x.into()),
            ctime: meta.created().ok().map(|x| x.into()),
            size: meta.len(),
        }
    }
}

/// Represents a single file that can be opened.
/// 
/// # Important
/// This [`DataFile`] only supports `async` operations. If you need
/// a blocking system, please use [`crate::blocking`] instead.
#[derive(Clone, Debug)]
pub struct DataFile {
    be: Arc<DataInner>,
    meta: Metadata,
}

impl PartialEq for DataFile {
    fn eq(&self, other: &Self) -> bool {
        self.be == other.be && self.meta == other.meta
    }
}

impl Eq for DataFile {}

impl DataFile {
    /// Constructs a new [`DataFile`] instance.
    ///
    /// # Arguments
    /// * `meta` - The [`Metadata`] to use.
    /// * `be` - The [`DataInner`] to use.
    pub(crate) fn new(meta: Metadata, be: Arc<DataInner>) -> Self {
        DataFile { meta, be }
    }

    /// # Returns
    /// The associated [`Metadata`] with this file.
    pub fn metadata(&self) -> Metadata {
        self.meta.clone()
    }

    /// # Returns
    /// The full, absolute [`Path`] of the node.
    pub fn path(&self) -> &Path {
        self.meta.path()
    }

    /// # Returns
    /// The [`Path::file_name`] of this file.
    pub fn name(&self) -> &OsStr {
        self.meta.name()
    }

    /// Opens the [`DataFile`] in read mode.
    /// 
    /// # Returns
    /// A [`DataRead`] handle to the file.
    pub async fn open_read(&self) -> io::Result<Box<dyn DataRead>> {
        self.be.open_read(&self.meta.path).await
    }

    /// Opens the [`DataFile`] in append mode.
    /// 
    /// # Returns
    /// A [`DataAppend`] handle to the file.
    pub async fn open_append(&self, truncate: bool) -> io::Result<Box<dyn DataAppend>> {
        self.be.open_append(&self.meta.path, truncate).await
    }

    /// Opens the [`DataFile`] in full read/write mode.
    /// 
    /// # Important
    /// This operation is not supported on remote backends, as it 
    /// requires [`Write`] + [`Seek`] access. The OpenDAL system
    /// does not allow this behavior.
    /// 
    /// # Returns
    /// A [`DataFull`] handle to the file.
    pub async fn open_full(&self) -> io::Result<Box<dyn DataFull>> {
        self.be.open_full(&self.meta.path).await
    }
}
