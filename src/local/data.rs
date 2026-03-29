use crate::backend::{UsageStat, VfsBackend, VfsReader, VfsWriter};
use crate::file::DataFile;
use crate::filters::FilterOptions;
use crate::local::handle::LocalHandle;
use crate::local::query::LocalQuery;
use crate::local::writer::LocalWriter;
use crate::meta::ExtMetadata;
use crate::query::DataQuery;
use async_trait::async_trait;
use chrono::{DateTime, Local};
use filetime::{FileTime, set_symlink_file_times};
use std::fmt::{Debug, Formatter};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use sysinfo::Disks;
use tokio::fs;
use uuid::Uuid;

#[derive(Debug)]
pub struct LocalBackend {
    pub(crate) id: Uuid,
    pub(crate) path: PathBuf,
}

impl LocalBackend {
    fn join_force(&self, p: &Path) -> PathBuf {
        crate::join_force(&self.path, p)
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

    async fn raw_metadata(&self, path: &Path) -> std::io::Result<Option<ExtMetadata>> {
        if !fs::try_exists(&path).await? {
            return Ok(None);
        }
        let meta = fs::metadata(&path).await?;
        let x_meta = ExtMetadata {
            path: Self::get_relative(&self.path, &path),
            is_dir: meta.is_dir(),
            mtime: meta.modified().ok().map(|x| x.into()),
            atime: meta.accessed().ok().map(|x| x.into()),
            ctime: meta.created().ok().map(|x| x.into()),
            size: meta.len(),
            can_write: true,
            ..Default::default()
        };
        Ok(Some(x_meta))
    }
}

#[async_trait]
impl VfsReader for LocalBackend {
    fn get_id(&self) -> Uuid {
        self.id
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

    async fn get_metadata(&self, item: &Path) -> std::io::Result<Option<ExtMetadata>> {
        let path = self.join_force(item);
        self.raw_metadata(&path).await
    }

    async fn read_dir(
        &self,
        item: &Path,
        opts: Option<FilterOptions>,
        recursive: bool,
        include_root: bool,
    ) -> std::io::Result<Arc<dyn DataQuery>> {
        let path = self.join_force(item);
        let ret = LocalQuery::new(&self.path, &path, opts, recursive, include_root)?;
        Ok(Arc::new(ret))
    }

    async fn realpath(&self, item: &Path) -> PathBuf {
        self.join_force(item)
    }

    async fn get_existing(&self, item: &Path) -> std::io::Result<Option<DataFile>> {
        let path = self.join_force(item);
        match self.raw_metadata(&path).await? {
            Some(meta) => {
                let handle =
                    LocalHandle::new(self.path.to_owned(), item.to_path_buf(), meta.is_dir);
                let ret = DataFile::new(meta, Arc::new(handle), true);
                Ok(Some(ret))
            }
            None => Ok(None),
        }
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
}

#[async_trait]
impl VfsBackend for LocalBackend {}
