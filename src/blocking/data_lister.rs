use rustic_core::{AsyncDataLister, DataFilterOptions, DataIterator, DataLister};
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::Handle;
use crate::compat::data_list::DataStreamCompat;

pub struct DataListerCompat {
    handle: Handle,
    list: Arc<dyn AsyncDataLister>,
}

impl DataListerCompat {
    pub fn new(handle: Handle, list: Arc<dyn AsyncDataLister>) -> Self {
        Self { handle, list }
    }
}

impl DataLister for DataListerCompat {
    fn size(&self) -> std::io::Result<Option<u64>> {
        self.handle.block_on(self.list.size())
    }

    fn options(&self) -> DataFilterOptions {
        self.list.options()
    }

    fn get_iter(self: Arc<Self>) -> std::io::Result<Box<dyn DataIterator>> {
        let iter = self.handle.block_on(self.list.clone().get_iter())?;
        let ret = DataStreamCompat::new(self.handle.clone(), iter);
        Ok(Box::new(ret))
    }

    fn path(&self) -> &Path {
        self.list.path()
    }
}
