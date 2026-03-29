#[derive(serde::Deserialize, serde::Serialize, Default, Clone, Debug)]
#[non_exhaustive]
/// [`FilterOptions`] allow to filter a source by various criteria.
pub struct FilterOptions {
    /// Glob pattern to exclude/include (can be specified multiple times)
    pub globs: Vec<String>,

    /// Same as --glob pattern but ignores the casing of filenames
    pub ignore_globs: Vec<String>,

    /// Treat the provided filename like a .gitignore file (can be specified multiple times)
    pub custom_ignore_files: Vec<String>,

    /// Maximum size of files to be backed up. Larger files will be excluded.
    pub exclude_larger_than: Option<u64>,
}
