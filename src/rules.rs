use std::fs::File;
use std::path::Path;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Rules {
    pub allow: HashMap<String, Vec<String>>,
}

impl Rules {
    /// Returns a vector of sibling directory names that code in the
    /// passed-in directory is disallowed to import.
    pub fn get_disallowed_siblings(&self, dir: &str) -> Option<Vec<String>> {
        let unique_dirs = self.extract_unique_dirs();
        let allowed_dirs = self.get_allowed_siblings(dir)?;
        let diff = find_difference(unique_dirs, allowed_dirs.clone());
        let diff = diff.into_iter().filter(|x| x != dir).collect::<Vec<_>>();
        Some(diff)
    }

    fn extract_unique_dirs(&self) -> Vec<String> {
        let mut unique_names = self.allow.keys().cloned().collect::<Vec<_>>();
        for names in self.allow.values() {
            for name in names {
                unique_names.push(name.clone());
            }
        }
        unique_names.sort();
        unique_names.dedup();
        unique_names
    }

    fn get_allowed_siblings(&self, dir: &str) -> Option<&Vec<String>> {
        self.allow.get(dir)
    }
}

pub fn get_dir_rules(dir_path: &Path) -> Option<Rules> {
    let rules_path = dir_path.join(".deplint.rules.json");
    let rules_result = read_rules_file(&rules_path);
    return rules_result.ok();
}

pub fn get_child_disallowed_imports(
    root: &Path,
    current: &Path,
    disallowed_imports: &Vec<String>,
    rules: &Option<Rules>,
    directory: &str,
) -> Vec<String> {
    let mut dir_disallowed_imports = disallowed_imports.clone();
    if let Some(rules) = rules {
        if let Some(disallowed_siblings) = rules.get_disallowed_siblings(&directory) {
            let new_disallowed_imports = disallowed_siblings
                .iter()
                .map(|s| current.join(s))
                .filter_map(|p| p.strip_prefix(root).ok().map(|p| p.to_path_buf()))
                .map(|p| p.to_str().expect("").to_string())
                .collect::<Vec<_>>();
            dir_disallowed_imports.extend(new_disallowed_imports);
        }
    }
    dir_disallowed_imports
}

fn find_difference(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn read_rules_file(path: &Path) -> Result<Rules, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;
    let rules: Rules = serde_json::from_str(&json_content)?;
    Ok(rules)
}
