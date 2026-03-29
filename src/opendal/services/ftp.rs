use std::collections::BTreeMap;
use serde_derive::{Deserialize, Serialize};
use crate::opendal::services::RemoteConfig;

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct FtpConfig {
    pub endpoint: String,
    pub root: String,
    pub username: String,
    pub password: String,
}

const ENDPOINT: &'static str = "endpoint";
const ROOT: &'static str = "root";
const USER: &'static str = "user";
const PASSWORD: &'static str = "password";

impl RemoteConfig for FtpConfig {
    fn to_map(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(ENDPOINT.to_string(), self.endpoint);
        map.insert(ROOT.to_string(), self.root);
        map.insert(USER.to_string(), self.username);
        map.insert(PASSWORD.to_string(), self.password);
        return map;
    }
}