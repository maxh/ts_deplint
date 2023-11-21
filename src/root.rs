use std::path::{Path, PathBuf};

pub fn find_package_json_directory(start_path: &Path) -> Option<PathBuf> {
    let mut current_path = start_path.to_owned();

    loop {
        let package_json_path = current_path.join("package.json");

        if package_json_path.exists() {
            return Some(current_path);
        }

        if let Some(parent) = current_path.parent() {
            if parent == current_path {
                // We've reached the root directory, stop searching
                break;
            }
            current_path = parent.to_owned();
        } else {
            // No parent directory found, stop searching
            break;
        }
    }

    None
}
