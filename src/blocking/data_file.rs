;use std::path::Path;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::runtime::Handle;
use crate::backend::{DataRead, FileOpener};
use crate::blocking::data_list::DataStreamCompat;
use crate::blocking::data_lister::DataListerCompat;
use crate::blocking::data_rw::DataReadWriteCompat;
use crate::blocking::data_seq::SeqWriteCompat;

pub struct FileOpCompat {
    handle: Handle,
    file: Arc<dyn FileOpener>
}

#[async_trait]
impl FileOpener for FileOpCompat {
    fn can_read(&self) -> bool {
        self.file.can_read()
    }

    fn can_append(&self) -> bool {
        self.file.can_append()
    }

    fn can_full(&self) -> bool {
        self.file.can_full()
    }

    async fn open_read(&self) -> std::io::Result<Box<dyn DataRead>> {
        self.handle.block_on(self.file.open_read())
    }

    fn open_append(&self, truncate: bool) -> std::io::Result<Box<dyn SeqWrite>> {
        let file = self.handle.block_on(self.file.open_append(truncate))?;
        let ret = SeqWriteCompat::new(self.handle.clone(), file);
        Ok(Box::new(ret))
    }

    fn open_full(&self) -> std::io::Result<Box<dyn DataReadWrite>> {
        let file = self.handle.block_on(self.file.open_full())?;
        let ret = DataReadWriteCompat::new(self.handle.clone(), file);
        Ok(Box::new(ret))
    }

    fn read_dir(&self) -> std::io::Result<Arc<dyn DataLister>> {
        let file = self.handle.block_on(self.file.read_dir())?;
        let ret = DataListerCompat::new(self.handle.clone(), file);
        Ok(Arc::new(ret))
    }

    fn delete(&self) -> std::io::Result<()> {
        self.handle.block_on(self.file.delete())
    }

    fn rename(&self, dest: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.file.rename(dest))
    }
}
