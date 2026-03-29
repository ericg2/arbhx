use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteConfig;

#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OneDriveConfig {
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub root: String,
}

const REFRESH_TOKEN: &'static str = "refresh_token";
const CLIENT_ID: &'static str = "client_id";
const CLIENT_SECRET: &'static str = "client_secret";
const ROOT: &'static str = "root";

impl RemoteConfig for OneDriveConfig {
    fn to_map(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(REFRESH_TOKEN.to_string(), self.refresh_token);
        map.insert(CLIENT_ID.to_string(), self.client_id);
        map.insert(CLIENT_SECRET.to_string(), self.client_secret);
        map.insert(ROOT.to_string(), self.root);
        return map;
    }
}