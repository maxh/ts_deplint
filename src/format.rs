use std::path::Path;

use crate::{files, rules, RULES_FILE_NAME};

pub fn format_rules_files(target: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let p = target.join(RULES_FILE_NAME);
    let r = rules::read_rules_file(&p);
    if r.is_ok() {
        rules::write_formatted_rules_file(&p, &r.unwrap())?;
    }
    // Recurse into directories.
    let map = files::list_files_and_directories(target)?;
    let directories = map.get("directories").unwrap();
    for directory in directories {
        format_rules_files(&target.join(directory))?;
    }
    Ok(())
}
