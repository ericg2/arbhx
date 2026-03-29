use crate::backend::{UsageStat, VfsBackend, VfsReader, VfsWriter};
use crate::file::DataFile;
use crate::filters::FilterOptions;
use crate::layers::VfsLayer;
use crate::meta::ExtMetadata;
use crate::query::DataQuery;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use std::fmt::{Debug, Formatter};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct SecureLayer {
    vfs: Arc<dyn VfsBackend>,
}

impl VfsLayer for SecureLayer {
    fn bind(&self, vfs: Arc<dyn VfsBackend>) -> Arc<dyn VfsBackend> {
        let ret = SecureLayer { vfs };
        Arc::new(ret)
    }
}

#[async_trait]
impl VfsReader for SecureLayer {
    fn get_id(&self) -> Uuid {
        self.vfs.get_id()
    }

    async fn get_usage(&self) -> Option<std::io::Result<UsageStat>> {
        self.vfs.get_usage().await
    }

    async fn get_metadata(&self, item: &Path) -> std::io::Result<Option<ExtMetadata>> {
        self.vfs.get_metadata(item).await
    }

    async fn read_dir(&self, item: &Path, opts: Option<FilterOptions>, recursive: bool, include_root: bool) -> std::io::Result<Arc<dyn DataQuery>> {
        todo!()
    }

    async fn realpath(&self, item: &Path) -> PathBuf {
        todo!()
    }

    async fn get_existing(&self, item: &Path) -> std::io::Result<Option<DataFile>> {
        todo!()
    }
}

#[async_trait]
impl VfsWriter for SecureLayer {
    async fn remove_dir(&self, dirname: &Path) -> std::io::Result<()> {
        todo!()
    }

    async fn remove_file(&self, filename: &Path) -> std::io::Result<()> {
        todo!()
    }

    async fn create_dir(&self, item: &Path) -> std::io::Result<()> {
        todo!()
    }

    async fn set_times(&self, item: &Path, mtime: DateTime<Local>, atime: DateTime<Local>) -> std::io::Result<()> {
        todo!()
    }

    async fn set_length(&self, item: &Path, size: u64) -> std::io::Result<()> {
        todo!()
    }

    async fn move_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        todo!()
    }

    async fn copy_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        todo!()
    }
}

impl VfsBackend for SecureLayer {}