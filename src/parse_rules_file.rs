use std::fs::File;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

#[derive(Serialize, Deserialize)]
pub struct Rules {
    pub allow: HashMap<String, Vec<String>>,
}

impl Rules {
    pub fn extract_unique_names(&self) -> Vec<String> {
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

    pub fn get_allow_rules(&self, key: &str) -> Option<&Vec<String>> {
        self.allow.get(key)
    }

    pub fn get_disallowed_rules(&self, key: &str) -> Option<Vec<String>> {
        let unique_names = self.extract_unique_names();
        let allow_rules = self.get_allow_rules(key)?;
        let diff = find_difference(unique_names, allow_rules.clone());
        Some(diff)
    }
}

fn find_difference(a: Vec<String>, b: Vec<String>) -> Vec<String> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

pub fn parse_rules_file(file_path: &str) -> Result<Rules, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let rules: Rules = serde_json::from_str(&json_content)?;

    Ok(rules)
}
