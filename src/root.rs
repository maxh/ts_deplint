use std::path::Path;

pub fn find_package_json_directory(start_path: &Path) -> Option<&Path> {
    start_path
        .ancestors()
        .find(|path| path.join("package.json").exists())
}
