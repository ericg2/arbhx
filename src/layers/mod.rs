mod secure;

use std::sync::Arc;
use async_trait::async_trait;
use crate::backend::{VfsBackend, VfsReader};

#[async_trait]
pub trait VfsLayer: Send + Sync + 'static {
    fn bind(&self, vfs: Arc<dyn VfsBackend>) -> Arc<dyn VfsBackend>;
}