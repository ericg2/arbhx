use std::collections::BTreeMap;
use opendal::Scheme;
use serde_derive::{Deserialize, Serialize};
use crate::remote::services::RemoteConfig;

/// Represents a BackBlaze B2 config. All fields are required.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct B2Config {
    /// The starting path to treat as `root` (`/`). No data will
    /// be visible outside of this directory, similar to an OpenSSH jail.
    pub root: String,
    /// The Application Key for the BackBlaze API.
    pub application_key: String,
    /// The Application Key (ID) for the BackBlaze API.
    pub application_key_id: String,
    /// The Bucket *Name* for the BackBlaze API.
    pub bucket: String,
    /// The Bucket *ID* for the BackBlaze API.
    pub bucket_id: String,
}

const ROOT: &'static str = "root";
const APP_KEY: &'static str = "application_key";
const APP_ID: &'static str = "application_key_id";
const BUCKET_NAME: &'static str = "bucket";
const BUCKET_ID: &'static str = "bucket_id";

impl RemoteConfig for B2Config {
    fn to_map(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(ROOT.to_string(), self.root);
        map.insert(APP_KEY.to_string(), self.application_key);
        map.insert(APP_ID.to_string(), self.application_key_id);
        map.insert(BUCKET_NAME.to_string(), self.bucket);
        map.insert(BUCKET_ID.to_string(), self.bucket_id);
        map
    }

    fn scheme(&self) -> Scheme {
        Scheme::B2
    }
}