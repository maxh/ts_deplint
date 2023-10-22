use std::error::Error;
use std::path::Path;
use std::{collections::HashSet, env};

use ts_deplint::{
    list_violations, pretty_print_violations, update_readme_with_diagram, Violation,
    RULES_FILE_NAME,
};

/// ts_deplint is a tool for linting TypeScript projects for disallowed imports.
///
/// Usage:
///
///    ts_deplint <command> <path1> <path2> ...
///
/// Commands:
///
///   lint     Lint the passed-in paths for disallowed imports.
///   diagram  Update the README.md file in the passed-in paths with a diagram of the disallowed imports.
///
/// Paths:
///
///  Paths can be either directories or files.
///
/// # Errors
///
/// This function will return an error if anything goes wrong.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <command> <path1> <path2> ...", args[0]);
        std::process::exit(1);
    }

    let command = &args[1];

    let paths: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    match command.as_str() {
        "lint" => {
            let mut all_violations: HashSet<Violation> = HashSet::new();
            for path in paths {
                let target = Path::new(path);
                if !target.exists() {
                    eprintln!("Target path '{}' does not exist.", path);
                    std::process::exit(1);
                }
                let violations = list_violations(target)?;
                all_violations.extend(violations);
            }
            if all_violations.len() > 0 {
                pretty_print_violations(all_violations);
                std::process::exit(2);
            }
        }
        "diagram" => {
            for path in paths {
                let target = Path::new(path);
                let rules_path = target.join(RULES_FILE_NAME);
                let readme_path = target.join("README.md");

                if !rules_path.exists() {
                    eprintln!(
                        "Rules file does not exist in the target directory '{}'.",
                        path
                    );
                    std::process::exit(1);
                }

                update_readme_with_diagram(&rules_path, &readme_path)?;
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'lint' or 'diagram'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
