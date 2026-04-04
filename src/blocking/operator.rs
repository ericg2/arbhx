use crate::backend::{DataUsage, VfsConfig};
use crate::blocking::full::FullCompat;
use crate::blocking::reader::ReadCompat;
use crate::blocking::writer::AppendCompat;
use crate::blocking::{CompatAppend, CompatFull, CompatRead};
use crate::blocking::{DataFile, DataQuery};
use crate::fs::{FilterOptions, Metadata};
use crate::DataMode;
use chrono::{DateTime, Local};
use std::io;
use std::ops::Deref;
use std::path::Path;
use tokio::runtime::{Handle, Runtime};

#[derive(Clone, Debug)]
pub struct Operator {
    vfs: crate::Operator,
    rt: Handle,
}

impl Operator {
    pub fn with_info(config: DataMode) -> std::io::Result<Self> {
        let vfs = crate::Operator::with_info(config)?;
        let rt = Runtime::new()?.handle().clone();
        Ok(Self { vfs, rt })
    }
    pub fn new<V: VfsConfig>(config: V) -> std::io::Result<Self> {
        let vfs = crate::Operator::new(config)?;
        let rt = Runtime::new()?.handle().clone();
        Ok(Self { vfs, rt })
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
    /// Retrieves the [`crate::DataFile`]
    pub fn into_file(self) -> DataFile {
        match self {
            FileAction::Created(x) => x,
            FileAction::Found(x) => x,
        }
    }
}

impl Operator {
    pub fn usage(&self) -> io::Result<Option<DataUsage>> {
        self.rt.block_on(self.vfs.usage())
    }
    pub fn stat(&self, item: impl AsRef<Path>) -> io::Result<Option<Metadata>> {
        self.rt.block_on(self.vfs.stat(item))
    }
    pub fn open_read(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn CompatRead>> {
        let handle = self.rt.block_on(self.vfs.open_read(item))?;
        let ret = ReadCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn open_full(&self, item: impl AsRef<Path>) -> io::Result<Box<dyn CompatFull>> {
        let handle = self.rt.block_on(self.vfs.open_full(item))?;
        let ret = FullCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn open_append(
        &self,
        item: impl AsRef<Path>,
        truncate: bool,
    ) -> io::Result<Box<dyn CompatAppend>> {
        let handle = self.rt.block_on(self.vfs.open_append(item, truncate))?;
        let ret = AppendCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn remove_dir(&self, dirname: impl AsRef<Path>) -> io::Result<()> {
        self.rt.block_on(self.vfs.remove_dir(dirname))
    }
    pub fn remove_file(&self, filename: impl AsRef<Path>) -> io::Result<()> {
        self.rt.block_on(self.vfs.remove_file(filename))
    }
    pub fn create_dir(&self, item: impl AsRef<Path>) -> io::Result<()> {
        self.rt.block_on(self.vfs.create_dir(item))
    }
    pub fn set_length(&self, item: impl AsRef<Path>, size: u64) -> io::Result<()> {
        self.rt.block_on(self.vfs.set_length(item, size))
    }
    pub fn move_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()> {
        self.rt.block_on(self.vfs.move_to(old, new))
    }
    pub fn copy_to(&self, old: impl AsRef<Path>, new: impl AsRef<Path>) -> io::Result<()> {
        self.rt.block_on(self.vfs.copy_to(old, new))
    }

    pub fn set_times(
        &self,
        item: impl AsRef<Path>,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        self.rt.block_on(self.vfs.set_times(item, mtime, atime))
    }

    pub fn list(
        &self,
        item: impl AsRef<Path>,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> io::Result<DataQuery> {
        let query = self
            .rt
            .block_on(self.vfs.list(item, opts, recursive, include_root))?
            .to_query();
        let ret = DataQuery::new(self.rt.clone(), self.vfs.be.clone(), query);
        Ok(ret)
    }

    pub fn get_existing(&self, item: impl AsRef<Path>) -> io::Result<DataFile> {
        let meta = self.stat(item)?.ok_or(io::ErrorKind::NotFound)?;
        let ret = DataFile::new(self.rt.clone(), meta, self.vfs.be.clone());
        Ok(ret)
    }

    pub fn ensure_file(&self, item: impl AsRef<Path>) -> io::Result<FileAction> {
        let path = item.as_ref();
        match self.stat(path)? {
            None => {
                self.set_length(path, 0)?;
                let file = self.get_existing(path)?;
                Ok(FileAction::Created(file))
            }
            Some(x) => {
                let file = self.get_existing(path)?;
                Ok(FileAction::Found(file))
            }
        }
    }
}
