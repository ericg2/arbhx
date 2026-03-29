use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;
use futures_lite::Stream;
use crate::file::DataFile;
use crate::filters::FilterOptions;

#[async_trait]
pub trait DataQuery: Sync + Send + Unpin {
    /// Returns the size of the source (in bytes).
    ///
    /// # Errors
    ///
    /// * If the size could not be determined.
    ///
    /// # Returns
    ///
    /// The size of the source, if it is known.
    async fn size(self: Arc<Self>) -> std::io::Result<Option<u64>>;

    /// Returns the filtering options of the source
    ///
    /// # Returns
    ///
    /// The [`DataFilterOptions`] if available.
    async fn options(&self) -> FilterOptions;

    /// Returns the iterator of the source
    ///
    /// # Returns
    ///
    /// The iterator.
    async fn get_iter(self: Arc<Self>) -> std::io::Result<Pin<Box<dyn DataStream>>>;

    /// Returns the `[Path]` of the lister root.
    ///
    /// # Returns
    ///
    /// The path.
    async fn path(&self) -> &Path;
}

#[async_trait]
pub trait DataStream:
Stream<Item = std::io::Result<DataFile>> + Send
{
}

impl<T> DataStream for T
where
    T: Stream<Item = std::io::Result<DataFile>> + Send
{}