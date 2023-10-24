use std::path::{Path, PathBuf};

pub fn find_package_json_directory(start_path: &Path) -> Option<PathBuf> {
    start_path.ancestors().find(|path| {
        path.join("package.json").exists()
    }).map(|path| path.join("package.json"))
}
