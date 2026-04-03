use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteSource;
use crate::opendal::throttle::Throttle;

/// The config for a remote source.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    /// The maximum # of open connections.
    pub max_threads: Option<u8>,
    /// The [`Throttle`] settings.
    pub bandwidth: Option<Throttle>,
    /// The [`RemoteSource`] to use.
    pub src: RemoteSource,
}