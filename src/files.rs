use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub struct FilesAndDirectories {
    pub files: Vec<PathBuf>,
    pub directories: Vec<PathBuf>,
}

pub fn list_files_and_directories(
    directory_path: &Path,
) -> io::Result<FilesAndDirectories> {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    for entry in fs::read_dir(directory_path)? {
        let path = entry?.path();

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            directories.push(path);
        }
    }

    Ok(FilesAndDirectories {
        files,
        directories,
    })
}
