use serde_derive::{Deserialize, Serialize};
use crate::{LocalConfig, RemoteConfig};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DataConfig {
    Local(LocalConfig),
    Remote(RemoteConfig),
}