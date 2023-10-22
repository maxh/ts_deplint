use std::env;
use std::error::Error;
use std::path::Path;

mod files;
mod imports;
mod initial;
mod root;
mod rules;
mod visit;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        std::process::exit(1);
    }

    let target = Path::new(&args[1]);
    if let Some(root) = root::find_package_json_directory(target) {
        let disallowed_imports =
            initial::get_initial_disallowed_imports(&root, target, vec![], &root);
        let violations = visit::visit_path(root.as_ref(), disallowed_imports, target)?;
        if violations.len() > 0 {
            println!("Found {} violations:", violations.len());
            std::process::exit(1);
        } else {
            println!("No violations found.");
        }
    } else {
        println!("No package.json found in any parent directory.");
    }
    Ok(())
}
