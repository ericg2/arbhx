use crate::compat::data_lister::DataListerCompat;
use crate::compat::vfs_write::VfsWriterCompat;
use rustic_core::{
    AsyncVfsReader, DataFile, DataLister, DataLocation, ExtMetadata, UsageStat, VfsReader,
    VfsWriter,
};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;
use uuid::Uuid;

#[derive(Debug)]
pub struct VfsReaderCompat {
    handle: Handle,
    read: Arc<dyn AsyncVfsReader>,
}

impl VfsReaderCompat {
    pub fn new(handle: Handle, read: Arc<dyn AsyncVfsReader>) -> Self {
        Self { handle, read }
    }
}

impl VfsReader for VfsReaderCompat {
    fn get_id(&self) -> Uuid {
        self.read.get_id()
    }

    fn get_usage(&self) -> Option<std::io::Result<UsageStat>> {
        self.handle.block_on(self.read.get_usage())
    }

    fn get_metadata(&self, item: &Path) -> std::io::Result<Option<ExtMetadata>> {
        self.handle.block_on(self.read.get_metadata(item))
    }

    fn read_dir(&self, item: &Path, recursive: bool) -> std::io::Result<Arc<dyn DataLister>> {
        let iter = self.handle.block_on(self.read.read_dir(item, recursive))?;
        let ret = DataListerCompat::new(self.handle.clone(), iter);
        Ok(Arc::new(ret))
    }

    fn get_existing(&self, item: &Path) -> std::io::Result<Option<DataFile>> {
        self.handle.block_on(self.read.get_existing(item))
    }
    fn get_writer(&self) -> Option<Arc<dyn VfsWriter>> {
        if let Some(x) = self.handle.block_on(self.read.get_writer()) {
            let ret = VfsWriterCompat::new(self.handle.clone(), x);
            Some(Arc::new(ret))
        } else {
            None
        }
    }
    fn upgrade(&self) -> Option<&dyn DataLocation> {
        self.read.upgrade() // 3-27-26: Naturally sync due to repo actions...
    }
}
