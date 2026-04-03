use std::path::PathBuf;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use crate::backend::{DataVfs, VfsConfig};
use crate::local::data::LocalBackend;
use crate::operator::DataInner;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LocalConfig {
    pub path: PathBuf
}

impl VfsConfig for LocalConfig {
    fn to_backend(self) -> std::io::Result<Arc<DataInner>> {
        let be = LocalBackend::new(self)?;
        Ok(Arc::new(be.to_inner()))
    }
}