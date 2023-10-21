use std::env;
use std::error::Error;

mod list_files_and_directories;
mod parse_rules_file;
mod ts_import_extractor;

mod visit_directory;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    visit_directory::visit_directory(path)?;
    Ok(())
}
