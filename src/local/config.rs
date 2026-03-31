use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LocalConfig {
    pub path: PathBuf
}