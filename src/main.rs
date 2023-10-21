use std::env;
use std::io;

mod ts_import_extractor;

use ts_import_extractor::read_imports_from_file;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // The file path will be the second argument (index 1)
    let file_path = &args[1];

    let imports = read_imports_from_file(file_path)?;

    println!("Imports:");
    for import in imports {
        println!("{}", import);
    }

    Ok(())
}

