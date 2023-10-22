use std::env;
use std::error::Error;
use std::path::Path;

mod files;
mod imports;
mod root;
mod rules;
mod visit;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    if let Some(root_path) = root::find_package_json_directory(path) {
        println!("Found package.json in: {:?}", root_path);
        visit::visit_path(root_path.as_ref(), vec![], path)?;
    } else {
        println!("No package.json found in any parent directory.");
    }
    Ok(())
}
