use std::path::Path;

use crate::{files, rules, RULES_FILE_NAME};

pub fn format_rules_file(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let r = rules::read_rules_file(target)?;
    rules::write_formatted_rules_file(target, &r)?;
    Ok(())
}

pub fn format_rules_files_recursively(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p = target.join(RULES_FILE_NAME);
    let r = rules::read_rules_file(&p);
    if r.is_ok() {
        rules::write_formatted_rules_file(&p, &r.unwrap())?;
    }
    // Recurse into directories.
    let files_and_directories = files::list_files_and_directories(target)?;
    for directory in files_and_directories.directories {
        format_rules_files_recursively(&target.join(directory))?;
    }
    Ok(())
}
