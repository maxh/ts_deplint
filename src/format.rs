use std::path::Path;

use crate::{files, rules, RULES_FILE_NAME};

pub fn format_rules_file(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let r = rules::read_rules_file(target)?;
    rules::write_formatted_rules_file(target, r)?;
    Ok(())
}

pub fn format_rules_files_recursively(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p = target.join(RULES_FILE_NAME);
    if let Ok(rules) = rules::read_rules_file(&p) {
        rules::write_formatted_rules_file(&p, rules)?;
    }
    // Recurse into directories.
    let files_and_directories = files::list_files_and_directories(target)?;
    for directory in &files_and_directories.directories {
        format_rules_files_recursively(&target.join(directory))?;
    }
    Ok(())
}
