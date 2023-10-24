use std::path::Path;
use std::fs;

use crate::{rules, RULES_FILE_NAME};

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
    let directories = fs::read_dir(&target)?.filter_map(|r| {
        if let Ok(entry) = r {
            if entry.path().is_file() {
                return Some(entry.path())
            }
        }

        None
    });

    for directory in directories {
        format_rules_files_recursively(&directory)?;
    }

    Ok(())
}
