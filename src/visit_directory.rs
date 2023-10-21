use std::error::Error;

use serde_json::to_string_pretty;

use crate::{list_files_and_directories::list_files_and_directories, parse_rules_file};

pub fn visit_directory(path: &str) -> Result<(), Box<dyn Error>> {
    let map = list_files_and_directories(path)?;
    let directories = map.get("directories").unwrap();
    let files = map.get("files").unwrap();

    println!("Directories:");
    for directory in directories {
        println!("- {}", directory);
    }

    println!("Files:");
    for file in files {
        println!("- {}", file);
    }

    let rules_path = format!("{}/.deplint.rules.json", path);
    let rules = parse_rules_file::parse_rules_file(&rules_path)?;
    // Print the body of the config file.
    println!("Rules:");
    let pretty_json = to_string_pretty(&rules).expect("Failed to serialize Rules struct");
    println!("{}", pretty_json);

    for file in files {
        if !file.ends_with(".ts") {
            continue;
        }
        let full_path = format!("{}/{}", path, file);
        println!("Imports from file: {}", file);
        let imports = crate::ts_import_extractor::read_imports_from_file(&full_path)?;
        for import in imports {
            println!("{}", import);
        }
    }

    Ok(())
}
