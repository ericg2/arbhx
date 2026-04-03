use crate::backend::{DataAppend, DataFull, DataRead};
use crate::local::reader::LocalReader;
use async_trait::async_trait;
use std::io;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::fs;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite, AsyncWriteExt, ReadBuf};

#[derive(Debug)]
pub struct LocalWriter {
    pub(crate) file: File,
    pub(crate) path: PathBuf,
}

impl LocalWriter {
    pub async fn sequential(
        path: impl AsRef<Path>,
        truncate: bool,
    ) -> std::io::Result<Box<dyn DataAppend>> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(false)
            .append(true)
            .truncate(truncate)
            .open(&path)
            .await?;
        let ret = LocalWriter { file, path };
        Ok(Box::new(ret))
    }
    pub async fn full(path: impl AsRef<Path>) -> std::io::Result<Box<dyn DataFull>> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .await?;
        let ret = LocalWriter { file, path };
        Ok(Box::new(ret))
    }
    pub async fn set_length(path: impl AsRef<Path>, size: u64) -> std::io::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .await?
            .set_len(size)
            .await?;
        Ok(())
    }
}

impl AsyncRead for LocalWriter {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_read(cx, buf)
    }
}

impl AsyncSeek for LocalWriter {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        let this = self.get_mut();
        Pin::new(&mut this.file).start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_complete(cx)
    }
}

impl AsyncWrite for LocalWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_shutdown(cx)
    }
}

#[async_trait]
impl DataRead for LocalWriter {}

#[async_trait]
impl DataFull for LocalWriter {}

#[async_trait]
impl DataAppend for LocalWriter {
    async fn close(&mut self) -> io::Result<()> {
        self.file.flush().await // *** just make sure it's flushed!
    }
}

