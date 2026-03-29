use std::path::Path;

mod data;
mod throttle;
mod handle;
mod reader;
mod writer;
mod query;
mod services;

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