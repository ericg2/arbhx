use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteConfig;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
pub struct S3Config {
    pub root: String,
    pub bucket: String,
    pub endpoint: String,
    pub region: String,
    pub access_key_id: String,
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
}