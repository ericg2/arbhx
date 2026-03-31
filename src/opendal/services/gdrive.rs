use std::collections::BTreeMap;
use opendal::Scheme;
use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteConfig;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct GDriveConfig {
    pub root: String,
    pub refresh_token: String,
    pub client_id: String,
    pub client_secret: String,
}

const ROOT: &'static str = "root";
const REFRESH_TOKEN: &'static str = "refresh_token";
const CLIENT_ID: &'static str = "client_id";
const CLIENT_SECRET: &'static str = "client_secret";

impl RemoteConfig for GDriveConfig {
    fn to_map(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(ROOT.to_string(), self.root);
        map.insert(REFRESH_TOKEN.to_string(), self.refresh_token);
        map.insert(CLIENT_ID.to_string(), self.client_id);
        map.insert(CLIENT_SECRET.to_string(), self.client_secret);
        return map;
    }

    fn scheme(&self) -> Scheme {
        Scheme::Gdrive
    }
}