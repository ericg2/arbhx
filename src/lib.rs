#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::must_use_candidate)]

use std::path::{Component, Path, PathBuf};

pub(crate) mod operator;
pub(crate) mod backend;
pub(crate) mod util;

#[cfg(feature = "blocking")]
pub mod blocking;

pub mod remote;
pub mod local;
pub mod fs;

pub use backend::{DataAppend, DataFull, DataRead};
pub use operator::{Operator, DataMode};

pub(crate) fn join_force(base: impl AsRef<Path>, p: impl AsRef<Path>) -> PathBuf {
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
