use std::collections::BTreeMap;
use opendal::Scheme;
use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteConfig;

/// Represents an Amazon S3 server config. All fields are required.
#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct S3Config {
    /// The starting path to treat as `root` (`/`). No data will
    /// be visible outside of this directory, similar to an OpenSSH jail.
    pub root: String,
    /// The bucket to use for the server.
    pub bucket: String,
    /// The endpoint to use for the server.
    pub endpoint: String,
    /// The region to use for the server.
    pub region: String,
    /// The access key ID for the server.
    pub access_key_id: String,
    /// The access key for the server.
    pub secret_access_key: String,
}

const ROOT: &'static str = "root";
const BUCKET: &'static str = "bucket";
const ENDPOINT: &'static str = "endpoint";
const REGION: &'static str = "region";
const ACCESS_KEY_ID: &'static str = "access_key_id";
const SECRET_ACCESS_KEY: &'static str = "secret_access_key";

impl RemoteConfig for S3Config {
    fn to_map(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(ROOT.to_string(), self.root);
        map.insert(BUCKET.to_string(), self.bucket);
        map.insert(ENDPOINT.to_string(), self.endpoint);
        map.insert(REGION.to_string(), self.region);
        map.insert(ACCESS_KEY_ID.to_string(), self.access_key_id);
        map.insert(SECRET_ACCESS_KEY.to_string(), self.secret_access_key);
        return map;
    }

    fn scheme(&self) -> Scheme {
        Scheme::S3
    }
}