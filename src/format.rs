use std::path::Path;

use crate::{files, rules, RULES_FILE_NAME};

pub fn format_rules_files(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Recursively find all directories with a RULES_FILE_NAME file
    // and format them.
    let mut directories = Vec::new();
    let mut current = target.to_path_buf();
    loop {
        let map = files::list_files_and_directories(&current)?;
        let files = map.get("files").unwrap();
        if files.contains(&RULES_FILE_NAME.to_string()) {
            directories.push(current.clone());
        }
        let directories = map.get("directories").unwrap();
        if directories.len() == 0 {
            break;
        }
        current = current.join(&directories[0]);
    }
    for directory in directories {
        let p = directory.join(RULES_FILE_NAME);
        let r = rules::read_rules_file(&p)?;
        rules::write_rules_file(&p, &r)?;
    }
    Ok(())
}
