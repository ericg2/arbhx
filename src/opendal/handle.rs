use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use async_trait::async_trait;
use opendal::Operator;
use crate::backend::{DataAppend, DataFull, DataRead, FileOpener};
use crate::opendal::path_to_str;
use crate::opendal::query::OpenDALQuery;
use crate::opendal::reader::OpenDALReader;
use crate::opendal::writer::OpenDALWriter;
use crate::query::DataQuery;

pub struct OpenDALHandle {
    pub(crate) operator: Operator,
    pub(crate) path: PathBuf,
    pub(crate) is_dir: bool,
}

impl OpenDALHandle {
    pub(crate) fn new(operator: Operator, path: &Path, is_dir: bool) -> Self {
        Self {
            operator,
            path: path.into(),
            is_dir,
        }
    }
}

#[async_trait]
impl FileOpener for OpenDALHandle {
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
        let ret = OpenDALReader::new(self.path.clone(), self.operator.clone()).await?;
        Ok(Box::new(ret))
    }

    async fn open_append(&self, truncate: bool) -> std::io::Result<Box<dyn DataAppend>> {
        let ret = OpenDALWriter::new(self.path.clone(), self.operator.clone(), truncate).await?;
        Ok(Box::new(ret))
    }

    async fn open_full(&self) -> std::io::Result<Box<dyn DataFull>> {
        Err(ErrorKind::Unsupported.into()) // *** OpenDAL does not support this mode!
    }

    async fn read_dir(&self) -> std::io::Result<Arc<dyn DataQuery>> {
        let path = path_to_str(&self.path, true);
        let ret = OpenDALQuery::new(self.operator.clone(), path, None, false, false)?;
        Ok(Arc::new(ret))
    }

    async fn delete(&self) -> std::io::Result<()> {
        let path = path_to_str(&self.path, self.is_dir);
        if self.is_dir {
            self.operator.remove_all(&path).await?;
        } else {
            self.operator.delete(&path).await?;
        }
        Ok(())
    }

    async fn rename(&self, dest: &Path) -> std::io::Result<()> {
        let src = path_to_str(&self.path, self.is_dir);
        let dst = path_to_str(dest, self.is_dir);
        self.operator.rename(&src, &dst).await?;
        Ok(())
    }
}
