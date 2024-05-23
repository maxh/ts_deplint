use std::path::Path;

use crate::{
    rules::{read_rules_file, write_formatted_rules_file},
    violations::ReferenceToNonexistentDirectory,
    DisallowedImportViolation, RULES_FILE_NAME,
};

pub fn fix_violation(
    root: &Path,
    violation: &DisallowedImportViolation,
) -> Result<(), Box<dyn std::error::Error>> {
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
    let rules_path = root.join(common_prefix).join(RULES_FILE_NAME);
    let mut rules = read_rules_file(&rules_path)?;
    let mut allow = rules.allow;
    let disallowed_imports = allow
        .entry(dir_after_common_prefix)
        .or_insert_with(Vec::new);
    disallowed_imports.retain(|i| i != "-");
    disallowed_imports.push(disallowed_after_common_prefix);
    disallowed_imports.sort();
    disallowed_imports.dedup();
    rules.allow = allow;
    write_formatted_rules_file(&rules_path, rules)
}

pub fn remove_reference_to_nonexistent_directory(
    root: &Path,
    issue: &ReferenceToNonexistentDirectory,
) -> Result<(), Box<dyn std::error::Error>> {
    let rules_file_path = Path::new(&issue.rules_file_path);
    let mut rules = read_rules_file(rules_file_path)?;
    rules.allow = rules
        .allow
        .into_iter()
        .flat_map(|(source, targets)| {
            if source == issue.directory_name {
                None
            } else {
                Some((
                    source,
                    targets
                        .into_iter()
                        .filter(|target| target != &issue.directory_name)
                        .collect(),
                ))
            }
        })
        .collect();
    write_formatted_rules_file(rules_file_path, rules)
}
