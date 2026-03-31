use std::path::{Component, Path, PathBuf};

pub(crate) mod backend;
pub(crate) mod local;
pub(crate) mod opendal;
pub(crate) mod util;
pub(crate) mod vfs;

pub use {
    local::config::LocalConfig,
    opendal::{config::RemoteConfig, services::*},
    vfs::*,
};

pub fn join_force(base: impl AsRef<Path>, p: impl AsRef<Path>) -> PathBuf {
    let mut out = PathBuf::from(base.as_ref());
    for comp in p.as_ref().components() {
        match comp {
            Component::Prefix(_) => {} // skip drive letters / UNC prefix
            Component::RootDir => {}   // skip leading /
            other => out.push(other.as_os_str()),
        }
    }
    out
}
