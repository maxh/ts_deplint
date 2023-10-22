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
        // remove the director itself from the diff
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

fn find_difference(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

pub fn read_rules_file(path: &Path) -> Result<Rules, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let rules: Rules = serde_json::from_str(&json_content)?;

    Ok(rules)
}
