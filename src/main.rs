use std::env;
use std::io;

mod list_files_and_directories;
mod ts_import_extractor;

use list_files_and_directories::list_files_and_directories;
use ts_import_extractor::read_imports_from_file;

fn main() -> io::Result<()> {
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
        println!("{}", directory);
    }

    println!("Files:");
    for file in files {
        println!("{}", file);
    }

    for file in files {
        let full_path = format!("{}/{}", path, file);
        println!("Imports from file: {}", file);
        let imports = read_imports_from_file(&full_path)?;
        for import in imports {
            println!("{}", import);
        }
    }

    Ok(())
}
