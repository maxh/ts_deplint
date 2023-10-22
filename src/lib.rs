use std::{error::Error, path::Path};

mod diagram;
mod disallowed;
mod files;
mod fix;
mod format;
mod root;
mod rules;
mod ts_reader;
mod violations;
mod visit;

pub use diagram::update_readme_with_diagram;
pub use fix::fix_violation;
pub use format::format_rules_files;
pub use root::find_package_json_directory;
pub use rules::RULES_FILE_NAME;
pub use violations::pretty_print_violations;
pub use violations::Violation;

pub fn list_violations(
    root: &Path,
    target: &Path,
    abort_on_violation: bool,
) -> Result<Vec<violations::Violation>, Box<dyn Error>> {
    let disallowed_imports = disallowed::get_initial_disallowed_imports(&root, target);
    let violations = visit::visit_path(
        root.as_ref(),
        &disallowed_imports,
        target,
        abort_on_violation,
    )?;
    Ok(violations)
}
