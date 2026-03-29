use crate::backend::FileOpener;
use crate::meta::ExtMetadata;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone)]
pub struct DataFile {
    pub(crate) handle: Arc<dyn FileOpener>,
    pub(crate) meta: ExtMetadata,
}

impl DataFile {
    pub(crate) fn new(meta: ExtMetadata, handle: Arc<dyn FileOpener>, can_write: bool) -> Self {
        DataFile {
            meta: meta.can_write(can_write),
            handle,
        }
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

    pub fn handle(&self) -> Arc<dyn FileOpener> {
        self.handle.clone()
    }
}
