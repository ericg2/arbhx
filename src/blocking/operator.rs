use crate::backend::UsageStat;
use crate::blocking::full::FullCompat;
use crate::blocking::reader::ReadCompat;
use crate::blocking::writer::AppendCompat;
use crate::blocking::DataQuery;
use crate::blocking::{CompatAppend, CompatFull, CompatRead};
use crate::{DataConfig, ExtMetadata, FilterOptions, LocalConfig, RemoteConfig};
use chrono::{DateTime, Local};
use std::io;
use std::path::Path;
use tokio::runtime::{Handle, Runtime};

#[derive(Clone, Debug)]
pub struct DataOperator {
    vfs: crate::DataOperator,
    rt: Handle,
}

impl DataOperator {
    pub fn local(config: LocalConfig) -> std::io::Result<Self> {
        Self::new(DataConfig::Local(config))
    }
    pub fn remote(config: RemoteConfig) -> std::io::Result<Self> {
        Self::new(DataConfig::Remote(config))
    }
    pub fn new(config: DataConfig) -> std::io::Result<Self> {
        let vfs = crate::DataOperator::new(config)?;
        let rt = Runtime::new()?.handle().clone();
        Ok(Self { vfs, rt })
    }
}

impl DataOperator {
    pub fn usage(&self) -> io::Result<Option<UsageStat>> {
        self.rt.block_on(self.vfs.usage())
    }
    pub fn stat(&self, item: &Path) -> io::Result<Option<ExtMetadata>> {
        self.rt.block_on(self.vfs.stat(item))
    }
    pub fn open_read(&self, item: &Path) -> io::Result<Box<dyn CompatRead>> {
        let handle = self.rt.block_on(self.vfs.open_read(item))?;
        let ret = ReadCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn open_full(&self, item: &Path) -> io::Result<Box<dyn CompatFull>> {
        let handle = self.rt.block_on(self.vfs.open_full(item))?;
        let ret = FullCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn open_append(&self, item: &Path, truncate: bool) -> io::Result<Box<dyn CompatAppend>> {
        let handle = self.rt.block_on(self.vfs.open_append(item, truncate))?;
        let ret = AppendCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
    pub fn remove_dir(&self, dirname: &Path) -> io::Result<()> {
        self.rt.block_on(self.vfs.remove_dir(dirname))
    }
    pub fn remove_file(&self, filename: &Path) -> io::Result<()> {
        self.rt.block_on(self.vfs.remove_file(filename))
    }
    pub fn create_dir(&self, item: &Path) -> io::Result<()> {
        self.rt.block_on(self.vfs.create_dir(item))
    }
    pub fn set_length(&self, item: &Path, size: u64) -> io::Result<()> {
        self.rt.block_on(self.vfs.set_length(item, size))
    }
    pub fn move_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.rt.block_on(self.vfs.move_to(old, new))
    }
    pub fn copy_to(&self, old: &Path, new: &Path) -> io::Result<()> {
        self.rt.block_on(self.vfs.copy_to(old, new))
    }
    pub fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> io::Result<()> {
        self.rt.block_on(self.vfs.set_times(item, mtime, atime))
    }
    pub fn list(
        &self,
        item: &Path,
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
}
