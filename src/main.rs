use clap::{Parser, Subcommand};
use std::collections::HashSet;
use std::error::Error;
use std::path::Path;

use ts_deplint::{
    find_package_json_directory, list_violations, pretty_print_violations, update_readme_diagram,
    update_readme_diagrams_recursively, Violation, RULES_FILE_NAME,
};

#[derive(Parser)]
#[clap(name = "ts_deplint")]
/// ts_deplint is a tool for linting TypeScript projects for disallowed imports.
struct Opt {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Lint(LintCommand),
    Diagram(DiagramCommand),
    Fix(FixCommand),
    Format(FormatCommand),
}

#[derive(Parser)]
#[clap(rename_all = "camel_case")]
/// Lint the passed-in paths for disallowed imports.
struct LintCommand {
    #[arg(last = true)]
    /// Paths can be either directories or files.
    paths: Vec<String>,
}

#[derive(Parser)]
#[clap(rename_all = "camel_case")]
/// Update the README.md file in the passed-in paths with a diagram of the disallowed imports.
struct DiagramCommand {
    #[arg(last = true)]
    /// Paths can be either directories or files.
    paths: Vec<String>,
}

#[derive(Parser)]
#[clap(rename_all = "camel_case")]
/// Fix the disallowed imports in the passed-in paths by adding allow rules.
struct FixCommand {
    #[arg(last = true)]
    /// Paths can be either directories or files.
    paths: Vec<String>,
}

#[derive(Parser)]
#[clap(rename_all = "camel_case")]
/// Format the rules files in the passed-in paths.
struct FormatCommand {
    #[arg(last = true)]
    /// Paths can be either directories or files.
    paths: Vec<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::parse();

    match opt.command {
        Commands::Lint(command) => run_lint_command(command),
        Commands::Diagram(command) => run_diagram_command(command),
        Commands::Fix(command) => run_fix_command(command),
        Commands::Format(command) => run_format_command(command),
    }
}

fn run_lint_command(command: LintCommand) -> Result<(), Box<dyn Error>> {
    let sample_path = Path::new(&command.paths[0]);
    let root = find_package_json_directory(sample_path)
        .ok_or("No package.json found in any parent directory.")?;

    let mut all_violations: HashSet<Violation> = HashSet::new();
    for path in command.paths.iter() {
        let target = Path::new(path);
        if !target.exists() {
            return Err(format!("Target path '{}' does not exist.", path).into());
        }
        let violations = list_violations(&root, target, false)?;
        all_violations.extend(violations);
    }

    if all_violations.len() > 0 {
        let count = all_violations.len();
        pretty_print_violations(all_violations);
        return Err(format!("{} violations.", count).into());
    }

    Ok(())
}

fn run_diagram_command(command: DiagramCommand) -> Result<(), Box<dyn Error>> {
    for path in command.paths.iter() {
        let target = Path::new(path);
        if target.ends_with(RULES_FILE_NAME) {
            let readme_path = target.parent().unwrap().join("README.md");
            update_readme_diagram(target, &readme_path)?;
        } else if target.is_dir() {
            update_readme_diagrams_recursively(&target)?;
        } else {
            return Err(format!("Target path '{}' is not a rules file or directory.", path).into());
        }
    }

    Ok(())
}

fn run_fix_command(command: FixCommand) -> Result<(), Box<dyn Error>> {
    let sample_path = Path::new(&command.paths[0]);
    let root = find_package_json_directory(sample_path)
        .ok_or("No package.json found in any parent directory.")?;

    let mut i = 0;
    for path in command.paths.iter() {
        loop {
            let target = Path::new(path);
            if !target.exists() {
                eprintln!("Target path '{}' does not exist.", path);
                std::process::exit(1);
            }
            let violations = list_violations(&root, target, true)?;
            if violations.len() == 0 {
                break;
            }
            for violation in violations {
                ts_deplint::fix_violation(&root, &violation)?;
            }
            i += 1;
            if i > 500 {
                return Err("Looped 500 times. Something is wrong.".into());
            }
        }
    }

    Ok(())
}

fn run_format_command(command: FormatCommand) -> Result<(), Box<dyn Error>> {
    for path in command.paths.iter() {
        let target = Path::new(path);
        if !target.exists() {
            return Err(format!("Target path '{}' does not exist.", path).into());
        }
        if target.ends_with(RULES_FILE_NAME) {
            ts_deplint::format_rules_file(target)?;
        } else {
            ts_deplint::format_rules_files_recursively(target)?;
        }
    }

    Ok(())
}
