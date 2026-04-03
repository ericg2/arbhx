use std::collections::BTreeMap;
use opendal::Scheme;
use serde_derive::{Deserialize, Serialize};
use crate::remote::services::RemoteConfig;

/// Represents a OneDrive config. All fields are required.
#[derive(Clone, Serialize, Deserialize, Eq, Hash, PartialEq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OneDriveConfig {
    /// The starting path to treat as `root` (`/`). No data will
    /// be visible outside of this directory, similar to an OpenSSH jail.
    pub root: String,
    /// The OAuth2 Refresh Token for the API.
    pub refresh_token: String,
    /// The OAuth2 Client ID for the API.
    pub client_id: String,
    /// The OAuth2 Client Secret for the API.
    pub client_secret: String,
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

    fn scheme(&self) -> Scheme {
        Scheme::Onedrive
    }
}