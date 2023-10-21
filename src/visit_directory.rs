use std::error::Error;

use crate::{list_files_and_directories, parse_rules_file};

pub fn visit_directory(disallowed_imports: Vec<String>, path: &str) -> Result<(), Box<dyn Error>> {
    // println!("Visiting directory: {}", path);
    // println!("Disallowed imports: {:?}", disallowed_imports);
    let map = list_files_and_directories::list_files_and_directories(path)?;
    let directories = map.get("directories").unwrap();
    let files = map.get("files").unwrap();
    let rules_path = format!("{}/.deplint.rules.json", path);
    let rules_result = parse_rules_file::parse_rules_file(&rules_path);
    let rules = rules_result.ok();
    for directory in directories {
        let full_path = format!("{}/{}", path, directory);
        let mut dir_disallowed_imports = disallowed_imports.clone();
        if let Some(rules) = &rules {
            let disallowed_rules_result = rules.get_disallowed_rules(&directory);
            if let Some(disallowed_rules) = disallowed_rules_result {
                let new_disallowed_imports = disallowed_rules
                    .iter()
                    .map(|rule| format!("{}/{}", path, rule))
                    .map(|rule| rule.replace("/Users/maxheinritz/loop-payments/backend/src", "src"))
                    .collect::<Vec<_>>();
                dir_disallowed_imports.extend(new_disallowed_imports);
            }
        }
        visit_directory(dir_disallowed_imports, &full_path)?;
    }
    if disallowed_imports.is_empty() {
        return Ok(());
    }
    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = format!("{}/{}", path, file);
        let imports = crate::ts_import_extractor::read_imports_from_file(&full_path)?;
        for import in imports {
            for disallowed_import in &disallowed_imports {
                if import.starts_with(disallowed_import) {
                    println!("{} imports {}", full_path, import);
                }
            }
        }
    }
    Ok(())
}
