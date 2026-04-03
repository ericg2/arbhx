use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};
use crate::Operator;

mod dropbox;
mod ftp;
mod gdrive;
mod onedrive;
mod s3;
mod b2;

pub use b2::*;
pub use dropbox::*;
pub use ftp::*;
pub use gdrive::*;
pub use onedrive::*;
pub use s3::*;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
#[serde(tag = "Type")]
#[non_exhaustive]
/// Represents an OpenDAL-powered source. This should be 
/// fed into an [`Operator`], and can be saved with all options.
pub enum RemoteSource {
    /// A BackBlaze B2 store with a [`B2Config`].
    B2(B2Config),
    /// A Dropbox store with a [`DropboxConfig`].
    Dropbox(DropboxConfig),
    /// An FTP store with an [`FtpConfig`].
    FTP(FtpConfig),
    /// A Google Drive store with a [`GDriveConfig`].
    Google(GDriveConfig),
    /// A OneDrive store with a [`OneDriveConfig`].
    OneDrive(OneDriveConfig),
    /// An Amazon S3 store with a [`S3Config`].
    S3(S3Config),
}

impl RemoteSource {
    /// Converts the [`RemoteSource`] into a [`BTreeMap`]. This is
    /// good for parsing the config into an OpenDAL system.
    /// 
    /// # Returns
    /// The [`BTreeMap`] representation.
    pub(crate) fn to_map(self) -> BTreeMap<String, String> {
        match self {
            RemoteSource::B2(x) => x.to_map(),
            RemoteSource::Dropbox(x) => x.to_map(),
            RemoteSource::FTP(x) => x.to_map(),
            RemoteSource::Google(x) => x.to_map(),
            RemoteSource::OneDrive(x) => x.to_map(),
            RemoteSource::S3(x) => x.to_map(),
        }
    }
    
    /// # Returns
    /// The [`opendal::Scheme`] for parsing.
    pub(crate) fn scheme(&self) -> opendal::Scheme {
        match self {
            RemoteSource::B2(x) => x.scheme(),
            RemoteSource::Dropbox(x) => x.scheme(),
            RemoteSource::FTP(x) => x.scheme(),
            RemoteSource::Google(x) => x.scheme(),
            RemoteSource::OneDrive(x) => x.scheme(),
            RemoteSource::S3(x) => x.scheme(),
        }
    }
}

/// Represents a valid, OpenDAL remote option.
trait RemoteConfig {
    /// Must convert the [`RemoteConfig`] into a [`BTreeMap`] suitable for OpenDAL.
    fn to_map(self) -> BTreeMap<String, String>;
    /// Must return a valid [`opendal::Scheme`].
    fn scheme(&self) -> opendal::Scheme;
}