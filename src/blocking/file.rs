use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::runtime::Handle;
use crate::{DataAppend, DataFull, DataRead, ExtMetadata};
use crate::blocking::{CompatAppend, CompatFull, CompatRead};
use crate::blocking::full::FullCompat;
use crate::blocking::reader::ReadCompat;
use crate::blocking::writer::AppendCompat;
use crate::vfs::DataInner;

#[derive(Clone)]
pub struct DataFile {
    pub(crate) rt: Handle,
    pub(crate) be: Arc<DataInner>,
    pub(crate) meta: ExtMetadata,
}

impl DataFile {
    pub(crate) fn new(rt: Handle, meta: ExtMetadata, be: Arc<DataInner>) -> Self {
        DataFile { rt, meta, be }
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

    pub fn open_read(&self) -> io::Result<Box<dyn CompatRead>> {
        let handle = self.rt.block_on(self.be.open_read(&self.meta.path))?;
        let ret = ReadCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    pub fn open_append(&self, truncate: bool) -> io::Result<Box<dyn CompatAppend>> {
        let handle = self.rt.block_on(self.be.open_append(&self.meta.path, truncate))?;
        let ret = AppendCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }

    pub fn open_full(&self) -> io::Result<Box<dyn CompatFull>> {
        let handle = self.rt.block_on(self.be.open_full(&self.meta.path))?;
        let ret = FullCompat::new(self.rt.clone(), handle);
        Ok(Box::new(ret))
    }
}
