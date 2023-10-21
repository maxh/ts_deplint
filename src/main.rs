use std::env;
use std::error::Error;

mod list_files_and_directories;
mod parse_rules_file;
mod ts_import_extractor;

use list_files_and_directories::list_files_and_directories;
use serde_json::to_string_pretty;
use ts_import_extractor::read_imports_from_file;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];

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
        let imports = read_imports_from_file(&full_path)?;
        for import in imports {
            println!("{}", import);
        }
    }

    Ok(())
}
