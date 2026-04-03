use crate::backend::{
    DataAppend, DataFull, DataRead, DataVfs, SizedQuery, UsageStat, VfsFull, VfsReader, VfsWriter,
};
use crate::local::config::LocalConfig;
use crate::local::query::LocalQuery;
use crate::local::reader::LocalReader;
use crate::local::writer::LocalWriter;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use filetime::{set_symlink_file_times, FileTime};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sysinfo::Disks;
use tokio::fs;
use uuid::Uuid;
use crate::fs::{FilterOptions, Metadata};
use crate::operator::{DataInner, DataMode};

#[derive(Debug, Clone)]
pub struct LocalBackend {
    pub(crate) id: Uuid,
    pub(crate) path: PathBuf,
    pub(crate) config: LocalConfig,
}

impl LocalBackend {
    fn join_force(&self, p: &Path) -> PathBuf {
        crate::join_force(&self.path, p)
    }

    pub fn new(config: LocalConfig) -> std::io::Result<Self> {
        Ok(Self {
            id: Uuid::new_v4(),
            path: config.path.to_path_buf(),
            config,
        })
    }

    pub(crate) fn get_relative(path: &Path, abs: &Path) -> PathBuf {
        match abs.strip_prefix(&path) {
            Ok(rel) => {
                if rel.as_os_str().is_empty() {
                    PathBuf::from("/") // treat same-as-base as root
                } else {
                    PathBuf::from("/").join(rel) // prepend "/" for your VFS style
                }
            }
            Err(_) => PathBuf::from(abs), // fallback: return original path if not under base
        }
    }

    async fn raw_metadata(&self, path: &Path) -> std::io::Result<Option<Metadata>> {
        if !fs::try_exists(&path).await? {
            return Ok(None);
        }
        let meta = fs::metadata(&path).await?;
        let x_meta = Metadata {
            path: Self::get_relative(&self.path, &path),
            is_dir: meta.is_dir(),
            mtime: meta.modified().ok().map(|x| x.into()),
            atime: meta.accessed().ok().map(|x| x.into()),
            ctime: meta.created().ok().map(|x| x.into()),
            size: meta.len(),
        };
        Ok(Some(x_meta))
    }
}

impl DataVfs for LocalBackend {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn to_inner(self) -> DataInner {
        let id = self.id;
        let ret = Arc::new(self.clone());
        DataInner {
            id,
            info: DataMode::Local(self.config),
            reader: Some(ret.clone()),
            writer: Some(ret.clone()),
            full: Some(ret.clone()),
        }
    }
}

#[async_trait]
impl VfsReader for LocalBackend {
    fn realpath(&self, item: &Path) -> PathBuf {
        self.join_force(item)
    }

    async fn get_usage(&self) -> Option<std::io::Result<UsageStat>> {
        let disks = Disks::new_with_refreshed_list();
        let ret = disks
            .iter()
            .find(|x| self.path.starts_with(x.mount_point()))
            .map(|disk| {
                let max_bytes = disk.total_space(); // total bytes
                let free_bytes = disk.available_space(); // free bytes
                let used_bytes = max_bytes - free_bytes;
                UsageStat {
                    used_bytes,
                    max_bytes,
                }
            })
            .ok_or(ErrorKind::Unsupported.into());
        Some(ret)
    }

    async fn open_read(&self, item: &Path) -> std::io::Result<Box<dyn DataRead>> {
        LocalReader::read_file(self.join_force(item)).await
    }

    async fn get_metadata(&self, item: &Path) -> std::io::Result<Option<Metadata>> {
        let path = self.join_force(item);
        self.raw_metadata(&path).await
    }

    async fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> std::io::Result<Arc<dyn SizedQuery>> {
        let path = self.join_force(item);
        let ret = LocalQuery::new(&self.path, &path, opts, recursive, include_root)?;
        Ok(Arc::new(ret))
    }
}

#[async_trait]
impl VfsWriter for LocalBackend {
    async fn remove_dir(&self, dirname: &Path) -> std::io::Result<()> {
        fs::remove_dir_all(self.join_force(dirname)).await
    }

    async fn remove_file(&self, filename: &Path) -> std::io::Result<()> {
        fs::remove_file(self.join_force(filename)).await
    }

    async fn create_dir(&self, item: &Path) -> std::io::Result<()> {
        fs::create_dir_all(self.join_force(item)).await
    }

    async fn set_times(
        &self,
        item: &Path,
        mtime: DateTime<Local>,
        atime: DateTime<Local>,
    ) -> std::io::Result<()> {
        set_symlink_file_times(
            self.join_force(item),
            FileTime::from_system_time(atime.into()),
            FileTime::from_system_time(mtime.into()),
        )?;
        Ok(())
    }

    async fn set_length(&self, item: &Path, size: u64) -> std::io::Result<()> {
        let path = self.join_force(item);
        LocalWriter::set_length(&path, size).await
    }

    async fn move_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        let p_old = self.join_force(old);
        let p_new = self.join_force(new);
        fs::rename(&p_old, &p_new).await?;
        Ok(())
    }

    async fn copy_to(&self, old: &Path, new: &Path) -> std::io::Result<()> {
        let p_old = self.join_force(old);
        let p_new = self.join_force(new);
        fs::copy(&p_old, &p_new).await?;
        Ok(())
    }

    async fn open_append(
        &self,
        item: &Path,
        truncate: bool,
    ) -> std::io::Result<Box<dyn DataAppend>> {
        LocalWriter::sequential(item, truncate).await
    }
}

#[async_trait]
impl VfsFull for LocalBackend {
    async fn open_full(&self, item: &Path) -> std::io::Result<Box<dyn DataFull>> {
        LocalWriter::full(item).await
    }
}
