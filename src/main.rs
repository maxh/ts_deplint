use std::io;

mod ts_import_extractor;

use ts_import_extractor::read_imports_from_file; 

fn main() -> io::Result<()> {
    let file_path = "/Users/maxheinritz/personal/prisma-lint/src/lint-prisma-source-code.ts";

    let imports = read_imports_from_file(file_path)?;

    println!("Imports:");
    for import in imports {
        println!("{}", import);
    }

    Ok(())
}

