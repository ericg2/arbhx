use std::collections::BTreeMap;

mod dropbox;
mod ftp;
mod gdrive;
mod onedrive;
mod s3;
mod b2;

use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::b2::B2Config;
use crate::opendal::services::dropbox::DropboxConfig;
use crate::opendal::services::ftp::FtpConfig;
use crate::opendal::services::gdrive::GDriveConfig;
use crate::opendal::services::onedrive::OneDriveConfig;
use crate::opendal::services::s3::S3Config;

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
}

trait RemoteConfig {
    fn to_map(self) -> BTreeMap<String, String>;
}