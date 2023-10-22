use std::path::Path;

use crate::rules;

pub fn get_initial_disallowed_imports(
    root: &Path,
    target: &Path,
    disallowed_imports: Vec<String>,
    current: &Path,
) -> Vec<String> {
    if target.eq(current) {
        return disallowed_imports;
    }
    let remainder = target
        .strip_prefix(current)
        .ok()
        .expect(format!("Failed to strip prefix {:?} from {:?}", current, target).as_str());
    let first_directory =
        get_first_directory(remainder).expect("Failed to read first directory from remainder.");
    let child_disallowed_imports = rules::get_child_disallowed_imports(
        root,
        current,
        &disallowed_imports,
        &rules::get_dir_rules(current),
        &first_directory,
    );
    return get_initial_disallowed_imports(
        root,
        target,
        child_disallowed_imports,
        &current.join(first_directory),
    );
}

fn get_first_directory(path: &Path) -> Option<String> {
    path.components()
        .nth(0)
        .and_then(|component| component.as_os_str().to_str().map(String::from))
}
