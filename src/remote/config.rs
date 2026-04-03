use std::collections::BTreeMap;
use std::io;
use std::io::ErrorKind;
use std::str::FromStr;
use std::sync::Arc;
use bytesize::ByteSize;
use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};
use crate::backend::{DataVfs, VfsConfig};
use crate::remote::data::OpenDALBackend;
use crate::remote::services::RemoteSource;
use crate::operator::DataInner;

/// The config for a remote source.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Hash)]
pub struct RemoteConfig {
    /// The maximum # of open connections.
    pub max_threads: Option<u8>,
    /// The [`Throttle`] settings.
    pub bandwidth: Option<Throttle>,
    /// The [`RemoteSource`] to use.
    pub src: RemoteSource,
}

impl VfsConfig for RemoteConfig {
    fn to_backend(self) -> io::Result<Arc<DataInner>> {
        let dal = OpenDALBackend::new(self)?;
        Ok(Arc::new(dal.to_inner()))
    }
}

/// Throttling parameters
///
/// Note: Throttle implements [`FromStr`] to read it from something like "10kiB,10MB"
#[derive(Debug, Clone, Copy, PartialEq, Eq, Setters, Serialize, Deserialize, Hash)]
pub struct Throttle {
    pub bandwidth: u32,
    pub burst: u32,
}

impl FromStr for Throttle {
    type Err = Box<io::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s
            .split(',')
            .map(|s| {
                ByteSize::from_str(s.trim())
                    .map_err(|err| io::Error::new(ErrorKind::InvalidInput, err))
            })
            .map(|b| -> io::Result<u32> {
                let byte_size = b?.as_u64();
                byte_size
                    .try_into()
                    .map_err(|err| io::Error::new(ErrorKind::InvalidInput, err))
            });

        let bandwidth = values
            .next()
            .transpose()?
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "No bandwidth given."))?;

        let burst = values
            .next()
            .transpose()?
            .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "No burst given."))?;

        Ok(Self { bandwidth, burst })
    }
}