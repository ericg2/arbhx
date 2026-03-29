use crate::backend::DataRead;
use crate::opendal::path_to_str;
use async_trait::async_trait;
use opendal::{FuturesAsyncReader, Operator};
use std::io::SeekFrom;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncSeek, ReadBuf};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt};

pub struct OpenDALReader {
    pub(crate) path: PathBuf,
    pub(crate) rdr: Compat<FuturesAsyncReader>,
}

impl OpenDALReader {
    pub(crate) async fn new(path: PathBuf, operator: Operator) -> std::io::Result<Self> {
        let f_path = path_to_str(&path, false);
        let rdr = operator
            .reader(&f_path)
            .await?
            .into_futures_async_read(..)
            .await?
            .compat();
        Ok(Self { path, rdr })
    }
}

impl AsyncRead for OpenDALReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.rdr).poll_read(cx, buf)
    }
}

impl AsyncSeek for OpenDALReader {
    fn start_seek(self: Pin<&mut Self>, position: SeekFrom) -> std::io::Result<()> {
        let this = self.get_mut();
        Pin::new(&mut this.rdr).start_seek(position)
    }

    fn poll_complete(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<u64>> {
        let this = self.get_mut();
        Pin::new(&mut this.rdr).poll_complete(cx)
    }
}

impl DataRead for OpenDALReader {}
