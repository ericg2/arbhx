use rustic_core::{AsyncVfsWriter, Metadata, VfsWriter};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;

pub struct VfsWriterCompat {
    handle: Handle,
    write: Arc<dyn AsyncVfsWriter>,
}

impl VfsWriterCompat {
    pub fn new(handle: Handle, write: Arc<dyn AsyncVfsWriter>) -> Self {
        Self { handle, write }
    }
}

impl VfsWriter for VfsWriterCompat {
    fn remove_dir(&self, dirname: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.write.remove_dir(dirname))
    }

    fn remove_file(&self, filename: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.write.remove_file(filename))
    }

    fn create_dir(&self, item: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.write.create_dir(item))
    }

    fn set_times(&self, item: &Path, meta: &Metadata) -> std::io::Result<()> {
        self.handle.block_on(self.write.set_times(item, meta))
    }

    fn set_length(&self, item: &Path, size: u64) -> std::io::Result<()> {
        self.handle.block_on(self.write.set_length(item, size))
    }

    fn move_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.write.move_to(old, new))
    }

    fn copy_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        self.handle.block_on(self.write.copy_to(old, new))
    }
}
