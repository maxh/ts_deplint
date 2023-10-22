use crate::{
    files, imports,
    rules::{self, Rules},
};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

pub fn visit_path(
    root: &Path,
    disallowed_imports: Vec<String>,
    target: &Path,
) -> Result<(), Box<dyn Error>> {
    let map = files::list_files_and_directories(target)?;
    let directories = map.get("directories").unwrap();
    let files = map.get("files").unwrap();
    let rules_path = target.join(".deplint.rules.json");
    let rules_result = rules::read_rules_file(&rules_path);
    let rules = rules_result.ok();

    visit_directories(root, &disallowed_imports, &rules, &target, &directories)?;
    check_files_for_disallowed_imports(root, &disallowed_imports, &target, &files)?;

    Ok(())
}

fn check_files_for_disallowed_imports(
    root: &Path,
    disallowed_imports: &Vec<String>,
    target: &Path,
    files: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = target.join(file);
        let relative_path = full_path.strip_prefix(root)?;
        let imports = imports::read_imports_from_file(&full_path)?;
        for import in imports {
            for disallowed_import in disallowed_imports {
                if import.starts_with(disallowed_import) {
                    println!(
                        "{} \n  imports from {}",
                        relative_path.to_str().expect(""),
                        disallowed_import,
                    );
                }
            }
        }
    }
    Ok(())
}

fn get_full_path(target: &Path, directory: &str) -> PathBuf {
    target.join(directory)
}

fn get_updated_disallowed_imports(
    root: &Path,
    target: &Path,
    disallowed_imports: &Vec<String>,
    rules: &Option<Rules>,
    directory: &str,
) -> Vec<String> {
    let mut dir_disallowed_imports = disallowed_imports.clone();
    if let Some(rules) = rules {
        if let Some(disallowed_siblings) = rules.get_disallowed_siblings(&directory) {
            let new_disallowed_imports = disallowed_siblings
                .iter()
                .map(|s| target.join(s))
                .filter_map(|p| p.strip_prefix(root).ok().map(|p| p.to_path_buf()))
                .map(|p| p.to_str().expect("").to_string())
                .collect::<Vec<_>>();
            dir_disallowed_imports.extend(new_disallowed_imports);
        }
    }
    dir_disallowed_imports
}

fn visit_directory(
    root: &Path,
    disallowed_imports: &Vec<String>,
    rules: &Option<Rules>,
    target: &Path,
    directory: &str,
) -> Result<(), Box<dyn Error>> {
    let full_path = get_full_path(target, directory);
    let dir_disallowed_imports =
        get_updated_disallowed_imports(root, target, disallowed_imports, rules, directory);
    visit_path(root, dir_disallowed_imports, &full_path)?;
    Ok(())
}

fn visit_directories(
    root: &Path,
    disallowed_imports: &Vec<String>,
    rules: &Option<Rules>,
    target: &Path,
    directories: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for directory in directories {
        visit_directory(root, disallowed_imports, rules, target, directory)?;
    }
    Ok(())
}
