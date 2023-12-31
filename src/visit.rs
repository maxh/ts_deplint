use crate::{disallowed, files, rules, ts_reader, violations::Violation};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub fn visit_path(
    root: &Path,
    disallowed_imports: &Vec<String>,
    current: &Path,
    abort_on_violation: bool,
) -> Result<Vec<Violation>, Box<dyn Error>> {
    let files_and_directories = files::list_files_and_directories(current)?;

    let mut violations = Vec::new();

    violations.extend(check_files_for_disallowed_imports(
        root,
        disallowed_imports,
        &current,
        &files_and_directories.files,
        abort_on_violation,
    )?);
    if abort_on_violation && violations.len() > 0 {
        return Ok(violations);
    }

    violations.extend(visit_directories(
        root,
        disallowed_imports,
        &current,
        &files_and_directories.directories,
        abort_on_violation,
    )?);

    Ok(violations)
}

fn check_files_for_disallowed_imports(
    root: &Path,
    disallowed_imports: &Vec<String>,
    current: &Path,
    files: &Vec<PathBuf>,
    abort_on_violation: bool,
) -> Result<Vec<Violation>, Box<dyn Error>> {
    let mut violations = Vec::new();

    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = current.join(file);
        let relative_path = full_path.strip_prefix(root)?;
        let imports = ts_reader::read_ts_imports(&full_path)?;
        for import in imports {
            for disallowed_import in disallowed_imports {
                if import.starts_with(disallowed_import) {
                    let violation = Violation {
                        file_path: relative_path.to_str().expect("").to_string(),
                        disallowed_import: disallowed_import.clone(),
                    };
                    violations.push(violation);
                    if abort_on_violation {
                        return Ok(violations);
                    }
                }
            }
        }
    }

    Ok(violations)
}

fn visit_directories(
    root: &Path,
    disallowed_imports: &Vec<String>,
    current: &Path,
    directories: &Vec<PathBuf>,
    abort_on_violation: bool,
) -> Result<Vec<Violation>, Box<dyn Error>> {
    let mut violations = Vec::new();

    let current_rules = rules::get_dir_rules(current);
    for child in directories {
        let dir_disallowed_imports = disallowed::get_child_disallowed_imports(
            root,
            current,
            disallowed_imports,
            &current_rules,
            child,
        );
        let next = current.join(child);
        violations.extend(visit_path(
            root,
            &dir_disallowed_imports,
            &next,
            abort_on_violation,
        )?);
        if abort_on_violation && violations.len() > 0 {
            return Ok(violations);
        }
    }

    Ok(violations)
}
