use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};

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
pub enum RemoteSource {
    B2(B2Config),
    Dropbox(DropboxConfig),
    FTP(FtpConfig),
    Google(GDriveConfig),
    OneDrive(OneDriveConfig),
    S3(S3Config),
}

impl RemoteSource {
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

trait RemoteConfig {
    fn to_map(self) -> BTreeMap<String, String>;
    fn scheme(&self) -> opendal::Scheme;
}