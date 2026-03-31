use super::operator::DataInner;
use crate::backend::{DataAppend, DataFull, DataRead};
use chrono::{DateTime, Local};
use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtMetadata {
    /// The full path of the file.
    pub path: PathBuf,
    /// If the item is a directory.
    pub is_dir: bool,
    /// Unix mtime (last modification time)
    pub mtime: Option<DateTime<Local>>,
    /// Unix atime (last access time)
    pub atime: Option<DateTime<Local>>,
    /// Unix ctime (last status change time)
    pub ctime: Option<DateTime<Local>>,
    /// Unix uid (user id)
    pub uid: Option<u32>,
    /// Unix gid (group id)
    pub gid: Option<u32>,
    /// Unix user name
    pub user: Option<String>,
    /// Unix group name
    pub group: Option<String>,
    /// Unix inode number
    pub inode: u64,
    /// Unix device id
    pub device_id: u64,
    /// Size of the node
    pub size: u64,
    /// Number of hardlinks to this node
    pub links: u64,
    /// If the source can be written to.
    pub can_write: bool,
}

impl ExtMetadata {
    pub fn unix_perms(&self) -> u16 {
        let mut perms = if self.is_dir {
            0o040000 // directory
        } else {
            0o100000 // regular file
        };
        if self.can_write {
            perms = perms | 0o666;
        } else {
            perms = perms | 0o444;
        }
        perms
    }
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct DataFile {
    pub(crate) be: Arc<DataInner>,
    pub(crate) meta: ExtMetadata,
}

impl DataFile {
    pub(crate) fn new(meta: ExtMetadata, be: Arc<DataInner>) -> Self {
        DataFile { meta, be }
    }

    pub fn metadata(&self) -> ExtMetadata {
        self.meta.clone()
    }

    pub fn path(&self) -> PathBuf {
        self.meta.path.clone()
    }

    pub fn name(&self) -> &OsStr {
        self.meta.path.file_name().unwrap_or_default()
    }

    pub async fn open_read(&self) -> io::Result<Box<dyn DataRead>> {
        self.be.open_read(&self.meta.path).await
    }

    pub async fn open_append(&self, truncate: bool) -> io::Result<Box<dyn DataAppend>> {
        self.be.open_append(&self.meta.path, truncate).await
    }

    pub async fn open_full(&self) -> io::Result<Box<dyn DataFull>> {
        self.be.open_full(&self.meta.path).await
    }
}
