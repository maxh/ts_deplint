use std::path::Path;

use crate::rules;

pub fn get_initial_disallowed_imports(root: &Path, target: &Path) -> Vec<String> {
    return get_initial_disallowed_imports_impl(root, target, vec![], &root);
}

fn get_initial_disallowed_imports_impl(
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
    let next_dir_name = remainder
        .components()
        .nth(0)
        .and_then(|component| component.as_os_str().to_str().map(String::from))
        .expect("Failed to read next directory name.");
    let child_disallowed_imports = rules::get_child_disallowed_imports(
        root,
        current,
        &disallowed_imports,
        &rules::get_dir_rules(current),
        &next_dir_name,
    );
    return get_initial_disallowed_imports_impl(
        root,
        target,
        child_disallowed_imports,
        &current.join(next_dir_name),
    );
}
