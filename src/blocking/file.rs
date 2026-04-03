use crate::blocking::full::FullCompat;
use crate::blocking::reader::ReadCompat;
use crate::blocking::writer::AppendCompat;
use crate::blocking::{CompatAppend, CompatFull, CompatRead};
use crate::vfs::DataInner;
use crate::Metadata;
use std::ffi::OsStr;
use std::io;
use std::io::{Seek, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::runtime::Handle;

/// Represents a single file that can be opened.
/// 
/// # Important
/// This [`DataFile`] does <b>NOT</b> support `async` operations. If you need this,
/// use the [`crate::DataFile`] (non-blocking) version.
/// 
/// # Equality
/// This [`DataFile`] implements both [`Eq`] and [`PartialEq`]. A file is considered
/// equal if the underlying [`Operator`] and [`Metadata`] fields are the same.
#[derive(Clone, Debug)]
pub struct DataFile {
    pub(crate) rt: Handle,
    pub(crate) be: Arc<DataInner>,
    pub(crate) meta: Metadata,
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
    /// * `rt` - The async [`Handle`] to use.
    /// * `meta` - The [`Metadata`] to use.
    /// * `be` - The [`DataInner`] to use.
    pub(crate) fn new(rt: Handle, meta: Metadata, be: Arc<DataInner>) -> Self {
        DataFile { rt, meta, be }
    }

    /// # Returns
    /// The associated [`Metadata`] with this file.
    pub fn metadata(&self) -> Metadata {
        self.meta.clone()
    }

    /// # Returns
    /// The full, absolute [`Path`] of the node.
    pub fn path(&self) -> PathBuf {
        self.meta.path.clone()
    }

    /// # Returns
    /// The [`Path::file_name`] of this file.
    pub fn name(&self) -> &OsStr {
        self.meta.path.file_name().unwrap_or_default()
    }

    /// Opens the [`DataFile`] in read mode.
    ///
    /// # Returns
    /// A [`CompatRead`] handle to the file.
    pub fn open_read(&self) -> io::Result<Box<dyn CompatRead>> {
        let handle = self.rt.block_on(self.be.open_read(&self.meta.path))?;
        let ret = ReadCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    /// Opens the [`DataFile`] in append mode.
    ///
    /// # Returns
    /// A [`CompatAppend`] handle to the file.
    pub fn open_append(&self, truncate: bool) -> io::Result<Box<dyn CompatAppend>> {
        let handle = self.rt.block_on(self.be.open_append(&self.meta.path, truncate))?;
        let ret = AppendCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    /// Opens the [`DataFile`] in full read/write mode.
    ///
    /// # Important
    /// This operation is not supported on remote backends, as it 
    /// requires [`Write`] + [`Seek`] access. The OpenDAL system
    /// does not allow this behavior.
    ///
    /// # Returns
    /// A [`CompatFull`] handle to the file.
    pub fn open_full(&self) -> io::Result<Box<dyn CompatFull>> {
        let handle = self.rt.block_on(self.be.open_full(&self.meta.path))?;
        let ret = FullCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
}
