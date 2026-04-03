use std::fmt::{Debug, Formatter};
use opendal::{FuturesAsyncWriter, Operator};
use opendal::options::WriteOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{Context, Poll};
use async_trait::async_trait;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use tokio_util::compat::{Compat, FuturesAsyncWriteCompatExt};
use crate::backend::DataAppend;
use crate::remote::path_to_str;

pub struct OpenDALWriter {
    pub(crate) path: PathBuf,
    pub(crate) wtr: Compat<FuturesAsyncWriter>,
}

impl Debug for OpenDALWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenDALWriter")
            .field("path", &self.path)
            .finish()
    }
}

impl OpenDALWriter {
    pub(crate) async fn new(path: PathBuf, operator: Operator, truncate: bool) -> io::Result<Self> {
        let f_path = path_to_str(&path, false);
        let wtr = operator
            .writer_options(
                &f_path,
                WriteOptions {
                    append: !truncate,
                    ..Default::default()
                },
            ).await?
            .into_futures_async_write().compat_write();
        Ok(Self { path, wtr })
    }
}

#[async_trait]
impl DataAppend for OpenDALWriter {
    async fn close(&mut self) -> io::Result<()> {
        self.wtr.shutdown().await?;
        Ok(())
    }
}

#[async_trait]
impl AsyncWrite for OpenDALWriter {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        Pin::new(&mut this.wtr).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.wtr).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        Pin::new(&mut this.wtr).poll_shutdown(cx)
    }
}
