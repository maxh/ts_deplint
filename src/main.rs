use std::env;
use std::error::Error;
use std::path::Path;

use ts_deplint::{list_violations, pretty_print_violations, update_readme_with_diagram};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: [lint|diagram] {} <path>", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let target = Path::new(&args[2]);
    if (command != "lint" && command != "diagram") || !target.exists() {
        eprintln!("Usage: [lint|diagram] {} <path>", args[0]);
        std::process::exit(1);
    }
    if command == "lint" {
        let violations = list_violations(target)?;
        pretty_print_violations(violations);
    }
    if command == "diagram" {
        update_readme_with_diagram(
            &target.join(".deplint.rules.yml"),
            &target.join("README.md"),
        )?;
    }
    Ok(())
}
