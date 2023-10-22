use std::error::Error;
use std::fs;
use std::path::Path;
use std::{collections::HashSet, env};

use ts_deplint::{
    list_violations, pretty_print_violations, update_readme_with_diagram, Violation,
    RULES_FILE_NAME,
};

/// Recursively find directories containing a rules file and update the diagram.
fn update_diagrams_recursively(dir: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if path.join(RULES_FILE_NAME).exists() {
                let readme_path = path.join("README.md");
                update_readme_with_diagram(&path.join(RULES_FILE_NAME), &readme_path)?;
            } else {
                update_diagrams_recursively(&path)?;
            }
        }
    }
    Ok(())
}

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
/// Examples:
///
///  ts_deplint lint src
///  ts_deplint lint src/domain src/app
///  ts_deplint lint src/domain/user.ts
///  ts_deplint diagram src/.deplint.rules.yml
///  ts_deplint diagram src
///  ts_deplint fix src
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
                let violations = list_violations(target, false)?;
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
                if target.ends_with(RULES_FILE_NAME) {
                    let readme_path = target.parent().unwrap().join("README.md");
                    update_readme_with_diagram(target, &readme_path)?;
                } else if target.is_dir() {
                    update_diagrams_recursively(&target)?;
                } else {
                    eprintln!("Target path '{}' is not a rules file or directory.", path);
                    std::process::exit(1);
                }
            }
        }
        "fix" => {
            // Loop. On each iteration find a violation in any path and fix it. Stop
            // when there are no more violations or when we've looped 500 times.
            let mut i = 0;
            for path in paths {
                loop {
                    let target = Path::new(path);
                    if !target.exists() {
                        eprintln!("Target path '{}' does not exist.", path);
                        std::process::exit(1);
                    }
                    println!("Looping through {}...", path);
                    let violations = list_violations(target, true)?;
                    if violations.len() == 0 {
                        break;
                    }
                    println!("Found {} violations.", violations.len());
                    for violation in violations {
                        ts_deplint::fix_violation(&violation)?;
                    }
                    i += 1;
                    if i > 500 {
                        eprintln!("Looped 500 times. Something is wrong.");
                        std::process::exit(1);
                    }
                }
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'lint' or 'diagram'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
