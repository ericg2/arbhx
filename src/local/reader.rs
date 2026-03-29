use std::io;
use std::io::SeekFrom;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncSeek, ReadBuf};
use crate::backend::DataRead;

#[derive(Debug)]
pub struct LocalReader {
    pub(crate) file: File,
    pub(crate) path: PathBuf
}

impl LocalReader {
    pub async fn read_file(path: impl AsRef<Path>) -> io::Result<Box<dyn DataRead>> {
        let path = path.as_ref().to_path_buf();
        let file = OpenOptions::new().read(true).write(false).append(false).open(&path).await?;
        let ret = LocalReader { path, file };
        Ok(Box::new(ret))
    }
}

#[async_trait]
impl DataRead for LocalReader {

}

impl AsyncRead for LocalReader {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_read(cx, buf)
    }
}

impl AsyncSeek for LocalReader {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> io::Result<()> {
        let this = self.get_mut();
        Pin::new(&mut this.file).start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        Pin::new(&mut this.file).poll_complete(cx)
    }
}