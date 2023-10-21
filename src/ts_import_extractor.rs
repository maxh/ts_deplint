use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "/Users/maxheinritz/personal/prisma-lint/src/lint-prisma-source-code.ts";

    let imports = read_imports_from_file(file_path)?;

    println!("Imports:");
    for import in imports {
        println!("{}", import);
    }

    Ok(())
}

fn read_imports_from_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut imports = Vec::new();

    for line in reader.lines() {
        let line = line?;

        if let Some(import) = extract_import(&line) {
            imports.push(import);
        }
    }

    Ok(imports)
}

fn extract_import(line: &str) -> Option<String> {
    if let Some(start) = line.find("from ") {
        let end = line[start + 6..].find(";").unwrap_or(line.len());
        let path = &line[start + 6..start + 6 + end];
        return Some(path.to_string());
    }
    None
}
