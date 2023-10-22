use crate::{
    files, imports,
    rules::{self, Rules},
};
use std::{error::Error, path::Path};

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
    check_files_for_disallowed_imports(&disallowed_imports, &target, &files)?;

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
        let full_path = target.join(directory);
        let mut dir_disallowed_imports = disallowed_imports.clone();
        if let Some(rules) = &rules {
            let disallowed_siblings_result = rules.get_disallowed_siblings(&directory);
            if let Some(disallowed_siblings) = disallowed_siblings_result {
                let new_disallowed_imports = disallowed_siblings
                    .iter()
                    .map(|s| format!("{}/{}", target.to_str().expect(""), s))
                    .map(|s| s.replace("/Users/maxheinritz/loop-payments/backend/src", "src"))
                    .collect::<Vec<_>>();
                dir_disallowed_imports.extend(new_disallowed_imports);
            }
        }
        visit_path(root, dir_disallowed_imports, &full_path)?;
    }
    Ok(())
}

fn check_files_for_disallowed_imports(
    disallowed_imports: &Vec<String>,
    target: &Path,
    files: &Vec<String>,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = target.join(file);
        let imports = imports::read_imports_from_file(&full_path)?;
        for import in imports {
            for disallowed_import in disallowed_imports {
                if import.starts_with(disallowed_import) {
                    println!("{} imports {}", full_path.to_str().expect(""), import);
                }
            }
        }
    }
    Ok(())
}