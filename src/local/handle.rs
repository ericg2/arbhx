use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use tokio::fs;
use tokio::io::AsyncRead;
use crate::backend::{DataAppend, DataFull, DataRead, FileOpener};
use crate::local::query::LocalQuery;
use crate::local::reader::LocalReader;
use crate::local::writer::LocalWriter;
use crate::query::DataQuery;

pub struct LocalHandle {
    pub(crate) abs: PathBuf,
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool
}

impl LocalHandle {
    pub fn new(abs: PathBuf, path: PathBuf, is_dir: bool) -> Self {
        Self { abs, path, is_dir }
    }
}

#[async_trait]
impl FileOpener for LocalHandle {
    fn can_read(&self) -> bool {
        true
    }

    fn can_append(&self) -> bool {
        true
    }

    fn can_full(&self) -> bool {
        true
    }

    async fn open_read(&self) -> std::io::Result<Box<dyn DataRead>> {
        LocalReader::read_file(&self.path).await
    }

    async fn open_append(&self, truncate: bool) -> std::io::Result<Box<dyn DataAppend>> {
        LocalWriter::sequential(&self.path, truncate).await
    }

    async fn open_full(&self) -> std::io::Result<Box<dyn DataFull>> {
        LocalWriter::full(&self.path).await
    }

    async fn read_dir(&self) -> std::io::Result<Arc<dyn DataQuery>> {
        let ret = LocalQuery::new(&self.abs, &self.path, None, false, true)?;
        Ok(Arc::new(ret))
    }

    async fn delete(&self) -> std::io::Result<()> {
        let path = crate::join_force(&self.abs, &self.path);
        if self.is_dir {
            fs::remove_dir_all(&path).await
        } else {
            fs::remove_file(&path).await
        }
    }

    async fn rename(&self, dest: &Path) -> std::io::Result<()> {
        let path = crate::join_force(&self.abs, &self.path);
        let dst = crate::join_force(&path, dest);
        fs::rename(&path, &dst).await
    }
}