use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteSource;
use crate::opendal::throttle::Throttle;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct RemoteConfig {
    pub max_threads: Option<u8>,
    pub bandwidth: Option<Throttle>,
    pub src: RemoteSource,
}