use std::path::Path;

mod data;
mod throttle;
mod reader;
mod writer;
mod query;
mod config;

pub mod services;
pub use config::{RemoteConfig, Throttle};

/// Converts a [`Path`] into an OpenDAL-supported [`String`].
/// 
/// # Arguments
/// * `p` - The [`Path`] to convert from.
/// * `is_dir` - If representing a directory or file.
/// 
/// # Returns
/// A valid [`String`] for OpenDAL use.
pub(crate) fn path_to_str(p: &Path, is_dir: bool) -> String {
    let mut r = String::from(p.to_str().unwrap());
    if !r.starts_with("/") {
        r = format!("/{r}")
    }
    if is_dir && !r.ends_with("/") {
        r += "/"
    } else if !is_dir && r.ends_with("/") {
        r = r.strip_suffix("/").unwrap_or(&r).to_string()
    }
    r.replace("\\", "/") // *** fix for windows-style directories
}