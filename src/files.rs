use std::fs;
use std::io;
use std::path::Path;

pub struct FilesAndDirectories {
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

pub fn list_files_and_directories(
    directory_path: &Path,
) -> io::Result<FilesAndDirectories> {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    for entry in fs::read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                if path.is_file() {
                    files.push(name_str.to_string());
                } else if path.is_dir() {
                    directories.push(name_str.to_string());
                }
            }
        }
    }

    Ok(FilesAndDirectories {
        files,
        directories,
    })
}
