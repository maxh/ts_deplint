use std::path::Path;

use crate::rules::{self, Rules};

pub fn get_initial_disallowed_imports(root: &Path, target: &Path) -> Vec<String> {
    println!("get_initial_disallowed_imports");
    println!("{:?}", target);
    return get_initial_disallowed_imports_impl(root, target, vec![], &root);
}

pub fn get_child_disallowed_imports(
    root: &Path,
    disallowed_imports: &Vec<String>,
    rules: &Option<Rules>,
    directory: &Path,
) -> Vec<String> {
    let mut dir_disallowed_imports = disallowed_imports.clone();
    println!("get_child_disallowed_imports");
    println!("{:?}", directory);
    if let Some(current) = directory.parent() {
        println!("parent");
        println!("{:?}", current);
        if let Some(file_name) = directory.file_name().and_then(|os_str| os_str.to_str()) {
            if let Some(rules) = rules {
                if let Some(disallowed_siblings) = rules.get_disallowed_siblings(file_name) {
                    let new_disallowed_imports = disallowed_siblings
                        .into_iter()
                        .map(|s| current.join(s))
                        .filter_map(|p| p.strip_prefix(root).ok().map(|p| p.to_path_buf()))
                        .map(|p| p.to_str().expect("").to_string())
                        .collect::<Vec<_>>();
                    dir_disallowed_imports.extend(new_disallowed_imports);
                }
            }
        }
    }

    return dir_disallowed_imports;
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

    let child_disallowed_imports = get_child_disallowed_imports(
        root,
        &disallowed_imports,
        &rules::get_dir_rules(current),
        target,
    );
    return get_initial_disallowed_imports_impl(root, target, child_disallowed_imports, target);
}
