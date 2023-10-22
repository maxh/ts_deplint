use std::path::Path;

use crate::rules::{read_rules_file, write_rules_file};
use crate::{Violation, RULES_FILE_NAME};

pub fn fix_violation(violation: &Violation) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = &violation.file_path;
    let disallowed_import = &violation.disallowed_import;
    let mut common_prefix = file_path
        .chars()
        .zip(disallowed_import.chars())
        .take_while(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect::<String>();
    // strip any chars after the last /
    if let Some(last_slash_idx) = common_prefix.rfind('/') {
        common_prefix.truncate(last_slash_idx + 1);
    }
    let dir_after_common_prefix = file_path
        .chars()
        .skip(common_prefix.len())
        .take_while(|c| *c != '/')
        .collect::<String>();
    let disallowed_after_common_prefix = disallowed_import
        .chars()
        .skip(common_prefix.len())
        .take_while(|c| *c != '/')
        .collect::<String>();
    let rules_path = Path::new(&common_prefix).join(RULES_FILE_NAME);
    let rules = read_rules_file(&rules_path);
    if let Ok(rules) = rules {
        let mut rules = rules;
        let mut allow = rules.allow;
        let disallowed_imports = allow
            .entry(dir_after_common_prefix)
            .or_insert_with(Vec::new);
        disallowed_imports.push(disallowed_after_common_prefix);
        disallowed_imports.sort();
        disallowed_imports.dedup();
        rules.allow = allow;
        return write_rules_file(&rules_path, &rules);
    }
    Ok(())
}
