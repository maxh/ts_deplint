use std::{error::Error, path::Path};

mod diagram;
mod disallowed;
mod files;
mod root;
mod rules;
mod ts_reader;
mod violations;
mod visit;

pub use diagram::update_readme_with_diagram;
pub use rules::RULES_FILE_NAME;
pub use violations::pretty_print_violations;
pub use violations::Violation;

pub fn list_violations(target: &Path) -> Result<Vec<violations::Violation>, Box<dyn Error>> {
    let root = root::find_package_json_directory(target)
        .ok_or("No package.json found in any parent directory.")?;
    let disallowed_imports = disallowed::get_initial_disallowed_imports(&root, target);
    let violations = visit::visit_path(root.as_ref(), &disallowed_imports, target)?;
    Ok(violations)
}
