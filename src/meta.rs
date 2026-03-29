use std::ffi::OsStr;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use derive_setters::Setters;
use serde_derive::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Setters)]
pub struct ExtMetadata {
    /// The full path of the file.
    pub path: PathBuf,
    /// If the item is a directory.
    pub is_dir: bool,
    /// Unix mtime (last modification time)
    pub mtime: Option<DateTime<Local>>,
    /// Unix atime (last access time)
    pub atime: Option<DateTime<Local>>,
    /// Unix ctime (last status change time)
    pub ctime: Option<DateTime<Local>>,
    /// Unix uid (user id)
    pub uid: Option<u32>,
    /// Unix gid (group id)
    pub gid: Option<u32>,
    /// Unix user name
    pub user: Option<String>,
    /// Unix group name
    pub group: Option<String>,
    /// Unix inode number
    pub inode: u64,
    /// Unix device id
    pub device_id: u64,
    /// Size of the node
    pub size: u64,
    /// Number of hardlinks to this node
    pub links: u64,
    /// If the source can be written to.
    pub can_write: bool,
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub x_attrs: Vec<ExtendedAttribute>,
}

//
// impl From<ExtMetadata> for Metadata {
//     fn from(value: ExtMetadata) -> Self {
//         Self {
//             mode: None,
//             mtime: value.mtime,
//             atime: value.atime,
//             ctime: value.ctime,
//             uid: value.uid,
//             gid: value.gid,
//             user: value.user,
//             group: value.group,
//             inode: value.inode,
//             device_id: value.device_id,
//             size: value.size,
//             links: value.links,
//             x_attrs: value.x_attrs,
//         }
//     }
// }

impl ExtMetadata {
    pub fn unix_perms(&self) -> u16 {
        let mut perms = if self.is_dir {
            0o040000 // directory
        } else {
            0o100000 // regular file
        };
        if self.can_write {
            perms = perms | 0o666;
        } else {
            perms = perms | 0o444;
        }
        perms
    }
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or_default()
    }
    // pub fn from_meta(path: impl AsRef<Path>, is_dir: bool, value: Metadata, can_write: bool) -> ExtMetadata {
    //     Self {
    //         is_dir,
    //         path: path.as_ref().to_path_buf(),
    //         mtime: value.mtime,
    //         atime: value.atime,
    //         ctime: value.ctime,
    //         uid: value.uid,
    //         gid: value.gid,
    //         user: value.user,
    //         group: value.group,
    //         inode: value.inode,
    //         device_id: value.device_id,
    //         size: value.size,
    //         links: value.links,
    //         x_attrs: value.x_attrs,
    //         can_write,
    //     }
    // }
}