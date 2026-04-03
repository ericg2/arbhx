use std::collections::BTreeMap;
use opendal::Scheme;
use serde_derive::{Deserialize, Serialize};
use crate::remote::services::RemoteConfig;

/// Represents an FTP server config. All fields are required.
#[derive(Clone, Serialize, Deserialize, Eq, PartialEq, Hash, Debug)]
pub struct FtpConfig {
    /// The IP address of the server. Example: `192.168.1.149:21`
    pub endpoint: String,
    /// The starting path to treat as `root` (`/`). No data will
    /// be visible outside of this directory, similar to an OpenSSH jail.
    pub root: String,
    /// The username to the FTP server.
    pub username: String,
    /// The password to the FTP server.
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

    fn scheme(&self) -> Scheme {
        Scheme::Ftp
    }
}